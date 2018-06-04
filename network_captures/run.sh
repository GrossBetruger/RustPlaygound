sudo setcap cap_net_admin,cap_net_raw+ep target/debug/network_captures
set x
sleep 0.3
cargo run
