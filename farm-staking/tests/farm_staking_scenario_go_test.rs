use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn adder_go() {
    world().run("scenarios/farm_staking.scen.json");
}

#[test]
fn interactor_trace_go() {
    world().run("scenarios/interactor_trace.scen.json");
}
