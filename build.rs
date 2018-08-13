#[cfg(feature = "optimize-cache")]
extern crate hwloc;

#[cfg(feature = "optimize-cache")]
fn get_l1_cache_size() -> usize{
    let topo = hwloc::Topology::new();
    let caches = topo.objects_with_type(&hwloc::ObjectType::Cache).unwrap();
    for cache in caches {
        let children = cache.children();
        children.iter().for_each(|ch| println!("cargo:children={}", ch));
        let size = cache.memory().local_memory();
        println!("cargo:size={}", size);
    }
    32000
}

fn main() {
    #[cfg(feature = "optimize-cache")]
    let cache_size = get_l1_cache_size();
}
