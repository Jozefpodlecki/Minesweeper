use anyhow::{anyhow, Result};
use crate::tests::TestCase;

pub fn should_have_title(expected: String) -> TestCase {
    TestCase {
        name: "should_have_title",
        run: Box::new(move |ctx| {
            let expected = expected.clone();
            Box::pin(async move {

                let title = ctx.driver.title().await?;
                if !title.contains(&expected) {
                    return Err(anyhow!("Title mismatch"));
                }

                Ok(())
            })
        }),
    }
}

pub fn should_() -> TestCase {
    TestCase {
        name: "should_show_play_button",
        run: Box::new(move |ctx| {
            // let expected = expected.clone();
            Box::pin(async move {


                Ok(())
            })
        }),
    }
}