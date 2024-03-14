use tokio::time::{sleep, Duration};

trait Advertisement {
    async fn run(&self);
    async fn render_fullscreen(&self);
    async fn remind_user_to_join_mailing_list(&self);
    async fn hide_for_now(&self);
}

struct Modal;

impl Advertisement for Modal {
    async fn run(&self) {
        self.render_fullscreen().await;
        for _ in 0..4u16 {
            self.remind_user_to_join_mailing_list().await;
        }
        self.hide_for_now().await;
    }

    async fn render_fullscreen(&self) {
        // Implementation for rendering fullscreen
        println!("Rendering fullscreen...");
        // Simulating some async operation with sleep
        sleep(Duration::from_secs(1)).await;
        println!("Rendered fullscreen.");
    }

    async fn remind_user_to_join_mailing_list(&self) {
        // Implementation for reminding the user to join the mailing list
        println!("Reminding user to join mailing list...");
        // Simulating some async operation with sleep
        sleep(Duration::from_secs(1)).await;
        println!("Reminded user to join mailing list.");
    }

    async fn hide_for_now(&self) {
        // Implementation for hiding the modal
        println!("Hiding modal for now...");
        // Simulating some async operation with sleep
        sleep(Duration::from_secs(1)).await;
        println!("Modal hidden for now.");
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world! async trait");
    let modal = Modal;
    modal.run().await;
}
