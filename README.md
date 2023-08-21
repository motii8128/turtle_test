# turtle_test
test program to move turtlesim by Rust(safe_drive)

## environment
|OS|ubuntu22.04|
|----|----|
|ros2_version|humble|

## How to use
### Make workspace
```
mkdir ros2_ws && cd ros2_ws
```
```
mkdir src && cd src
```

### Clone this code in "src" 
```
git clone https://github.com/motii8128/turtle_test.git
```

### Build
```
cd ros2_ws
```
```
colcon build --cargo-args
```
```
source /opt/ros/humble/setup.bash
```
```
. install/setup.bash
```
### Run
run turtlesim
```
ros2 run turtlesim turtlesim_node
```
```
ros2 run turtle_test turtle_test
```
