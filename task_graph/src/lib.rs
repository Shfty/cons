mod trait_sandbox;
pub use trait_sandbox::*;

pub use futures;

#[macro_export]
macro_rules ! run_serial {
        ($($task:expr),* $(,)?) => {
            async {
                $(
                    $task.await;
                )*
            }
        };
    }

#[macro_export]
macro_rules ! run_parallel {
        ($($task:expr),* $(,)?) => {
            async {
                $crate::futures::join!(
                    $($task),*
                );
            }
        }
    }

#[macro_export]
macro_rules ! task_graph {
    (
        (
            $(
                $e:tt
            ),*
        )
    ) => {
        $crate::run_parallel!(
            $(
                $crate::task_graph!($e)
            ),*
        )
    };
    (
        [
            $(
                $e:tt
            ),*
        ]
    ) => {
        $crate::run_serial!(
            $(
                $crate::task_graph!($e)
            ),*
        )
    };
    (
        [
            $(
                $e:expr
            ),*
        ]
    ) => {
        $crate::run_serial!(
            $(
                $e
            ),*
        )
    };
    (
        $(
            $e:tt
        ),*
    ) => {
        $crate::run_parallel!(
            $(
                $crate::task_graph!($e)
            ),*
        )
    };
}

// Wraps Task::run inside a future to allow execution inside the task graph
// Workable for API, but precludes tasks inside the graph from doing anything async - not ideal
pub async fn async_task<T>(t: &mut T)
where
    T: AsyncTask,
{
    t.run()
}

pub trait AsyncTask {
    fn run(&mut self);
}

struct PointSpriteUpdate;
impl AsyncTask for PointSpriteUpdate {
    fn run(&mut self) {
        println!("Point Sprite Update")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_async_task_trait() {
        let mut point_sprite_update = PointSpriteUpdate;
        let task = async_task(&mut point_sprite_update);
        async_std::task::block_on(task);
    }

    // Video Tasks
    async fn point_sprite_update() {
        println!("Point Sprite Update");
    }

    async fn point_sprite_render() {
        println!("Point Sprite Render");
    }

    async fn quad_update() {
        println!("Quad Update");
    }

    async fn quad_render() {
        async_std::task::sleep(Duration::from_secs_f64(1.5)).await;
        println!("Quad Render");
    }

    async fn mesh_update() {
        async_std::task::sleep(Duration::from_secs_f64(1.0)).await;
        println!("Mesh Update");
    }

    async fn mesh_render() {
        println!("Mesh Render");
    }

    async fn composite_framebuffer() {
        async_std::task::sleep(Duration::from_secs_f64(0.5)).await;
        println!("Composite Framebuffer")
    }

    async fn video_output() {
        println!("Video Output")
    }

    // Audio Tasks
    async fn audio_update() {
        println!("Audio Update")
    }
    async fn audio_render() {
        println!("Audio Render")
    }
    async fn audio_output() {
        println!("Audio Output")
    }

    #[test]
    fn test_task_graph() {
        async_std::task::block_on(task_graph!(
            [
                (
                    [point_sprite_update(), point_sprite_render()],
                    [quad_update(), quad_render()],
                    [mesh_update(), mesh_render()]
                ),
                [composite_framebuffer(), video_output()]
            ],
            [audio_update(), audio_render(), audio_output()]
        ));
    }
}
