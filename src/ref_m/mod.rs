pub mod ref_init;
pub mod enum_list;
pub mod ref_list;
pub mod my_box;
pub mod tree_list;
// BOX 应用场景
// 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
// 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
