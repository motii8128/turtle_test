use safe_drive::{context::Context, error::DynError, logger::Logger, pr_info, msg::common_interfaces::geometry_msgs};
use std::time::Duration;

// てすと

fn main()->Result<(), DynError>
{
    let context = Context::new()?;

    let node = context.create_node("turtle_test", None, Default::default())?;

    let publisher = node.create_publisher::<geometry_msgs::msg::Twist>("turtle1/cmd_vel", None)?;

    let logger = Logger::new("turtle_test");

    let mut msg = geometry_msgs::msg::Twist::new().unwrap();

    let mut value = 1.0;

    loop
    {
        msg.linear.x = value;

        pr_info!(logger, "kame ugoke");
        publisher.send(&msg)?;

        value = value * -1.0;

        std::thread::sleep(Duration::from_millis(1000));
    }
}
