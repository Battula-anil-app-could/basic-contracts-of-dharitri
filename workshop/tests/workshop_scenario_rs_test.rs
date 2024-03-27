use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("mxsc:output/workshop.mxsc.json", workshop::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() {
    world().run("scenarios/workshop.scen.json");
}

#[test]
fn interactor_trace_rs() {
    world().run("scenarios/interactor_trace.scen.json");
}
