use stix::Object;

fn main() {
    let enterprise = attck::enterprise();
    for tool in enterprise.tools() {
        println!("{} ({})", tool.name(), tool.id());
        for actor in tool.actors() {
            println!("  Used by {} ({})", actor.name(), actor.id());
        }
    }
}
