use macros::byteos_module_use;
/*
 * 这个文件内容定义了需要使用的模块，如果需要使用某个模块，取消某个模块的注释即可
 * 目前支持的模块如下
 */

// Tasks
byteos_module_use!(test_tasks);

// Filesystem
byteos_module_use!(ramfs);
byteos_module_use!(fatfs);
