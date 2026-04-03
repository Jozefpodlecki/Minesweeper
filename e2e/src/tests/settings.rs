use std::time::Duration;

use thirtyfour::{prelude::ElementQueryable, By};

use crate::tests::{TestCase, TestContext};

pub fn should_show_settings_widget() -> TestCase {
    TestCase {
        name: "should_show_settings_widget",
        run: Box::new(move |context: TestContext| {

            Box::pin(async move {

                let element = context.driver
                    .query(By::Css("[data-settings]"))
                    .wait(Duration::from_secs(2), Duration::from_millis(200))
                    .first()
                    .await?;

                element.click().await?;

                Ok(())
            })
        }),
    }
}

pub fn should_show_settings_modal() -> TestCase {
    TestCase {
        name: "should_show_settings_modal",
        run: Box::new(move |context: TestContext| {

            Box::pin(async move {

                let element = context.driver
                    .query(By::Css("data-settings"))
                    .wait(Duration::from_secs(2), Duration::from_millis(200))
                    .first()
                    .await?;

                element.click().await?;

                let element = context.driver
                    .query(By::Css("data-overlay"))
                    .wait(Duration::from_secs(2), Duration::from_millis(200))
                    .first()
                    .await?;

                Ok(())
            })
        }),
    }
}