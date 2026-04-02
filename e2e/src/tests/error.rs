use std::time::Duration;

use thirtyfour::{prelude::ElementQueryable, By};

use crate::tests::{TestCase, TestContext};


pub fn should_show_error_popup() -> TestCase {
    TestCase {
        name: "should_show_error_popup",
        run: Box::new(move |context: TestContext| {

            Box::pin(async move {

                let error_url = format!("{}#/error", context.app.base_url());
                context.driver.goto(error_url).await?;
                context.driver.set_implicit_wait_timeout(Duration::from_secs(5)).await?;

                let element = context.driver
                    .query(By::Css(""))
                    .wait(Duration::from_secs(5), Duration::from_millis(1000))
                    .first()
                    .await?;

                assert!(element.is_displayed().await?);

                Ok(())
            })
        }),
    }
}