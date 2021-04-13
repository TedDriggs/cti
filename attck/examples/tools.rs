use stix::Object;

fn main() {
    let enterprise = attck::enterprise();
    for tool in enterprise.tools() {
        println!("{} ({})", tool.name(), tool.id());
        for actor in tool.used_by_intrusion_sets() {
            println!("  Used by {} ({})", actor.name(), actor.id());
        }
    }
}
