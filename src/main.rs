use takus::state::GlobalState;
use takus::workflow::Workflow;

fn main() -> anyhow::Result<()> {
    let wf = Workflow::load("takus.yml")?;
    let gs = GlobalState::new()?;
    wf.run(&gs)
}
