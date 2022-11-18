//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

// TaskInfo容器
#[derive(Clone, Copy)]
pub struct TaskInfoInner {
    pub syscall_times: [u32; MAX_SYSCALL_NUM], // MAX_SYSCALL_NUM数据个数
    pub start_time: usize,
}

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    // 维护TaskInfo对象
    pub task_info_inner: TaskInfoInner,
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
