use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("file:output/farm-staking.wasm", farm_staking::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() {
    world().run("scenarios/farm_staking.scen.json");
}

#[test]
fn interactor_trace_rs() {
    world().run("scenarios/interactor_trace.scen.json");
}
