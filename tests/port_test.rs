use portsage::port::parse_lsof_output;

#[test]
fn test_parse_lsof_output() {
    let mock_output = r#"
COMMAND     PID USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
uvicorn    1234 user   10u  IPv4 0x12345678      0t0  TCP *:8000 (LISTEN)
node       5678 user   20u  IPv4 0x23456789      0t0  TCP *:3000 (LISTEN)
docker-pr  9012 user   22u  IPv6 0x34567890      0t0  TCP *:5432 (LISTEN)
"#;

    let map = parse_lsof_output(mock_output);

    assert_eq!(map.get(&8000), Some(&1234));
    assert_eq!(map.get(&3000), Some(&5678));
    assert_eq!(map.get(&5432), Some(&9012));
    assert_eq!(map.get(&9999), None);
}
