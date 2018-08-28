extern crate hierarchy;

use hierarchy::Hierarchy;

#[test]
fn test_hierarchy() {

    let a = Hierarchy::new();
    let root = a.add_node(1);
    let subroot1 = root.add_node(8);
    let subroot2 = root.add_node(9);


//    let zzz = vec![1,2,3];
//    zzz.len()
    assert_eq!(3, a.len());
    assert_eq!(&8, a.get(subroot1));
//    assert_eq!([1,8,9], a.iter().cloned)



}


#[test]
fn test_iteration() {
    let a = Hierarchy::new();
    let root = a.add_node(1);
    let subroot1 = root.add_node(8);
    let subroot2 = root.add_node(9);


    let mut root_iter = a.iter();

    for i in a.iter() {

    }

}