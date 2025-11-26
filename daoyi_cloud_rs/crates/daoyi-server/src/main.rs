fn main() {
    daoyi_gateway::mount_gateway();
    daoyi_framework::init_framework();
    println!("daoyi-server skeleton aggregator ready");
}
