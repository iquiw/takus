use takus::state::GlobalState;
use takus::workflow::Workflow;

fn main() -> anyhow::Result<()> {
    let mut wf = Workflow::load("takus.yml")?;
    let gs = GlobalState::new()?;
    let args = std::env::args().skip(1).collect();
    wf.run(&gs, &args)
}
