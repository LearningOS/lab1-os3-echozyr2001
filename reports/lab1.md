# Lab1

## 实现fn sys_task_info(ti *mut TaskInfo) -> isize

### 需求分析

该系统调用需要能够查询当前正在执行的任务信息，包括**任务状态**、**任务使用的系统调用以及调用次数**、**任务运行的总时长**。

 ```rust
struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize
}
 ```

从上面的描述中，我们可以获取以下信息：

1. 正在运行的任务表示任务状态一定是Running。
2. 每个`task`至少需要增加`syscall_times`字段和`time`字段。
3. 从`syscall_times`的定义可以看出，是在提示我们用`syscall`的`id`做索引来统计对应系统调用的调用次。

### 具体实现

* 由于任务状态一定是Running因此可以不用记录直接赋值；为了记录任务总运行时间，可以记录任务开始的时间，然后用任务结束的时间减去任务开始的时间即可。

* 综上所述，我们需要记录的就是任务使用的系统调用以及调用次数`syscall_times`和任务开始的时间`start_time`，参考TaskManager将`syscall_times`和`start_time`封装成TaskInfoInner。

* 然后给Task对象加入一个task_info_inner字段。

* 之后给TaskManager实现对TaskInfo的getter和setter

* 然后对外部提供接口

* 最后在使用系统调用的时候记录，并且完善目的系统调用

## 简答作业

1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 [Rust 三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rust-based-os-comp2022/tree/main/user/src/bin) ， 注意在编译时至少需要指定 `LOG=ERROR` 才能观察到内核的报错信息) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

    * 解答：RustSBI version 0.3.0-alpha.4；

        `bad_address`、 `bad_instruction` 和 `bad_register`触发的异常分别为：

        ```shell
        [ERROR] [kernel] PageFault in application, core dumped.
        [ERROR] [kernel] IllegalInstruction in application, core dumped.
        [ERROR] [kernel] IllegalInstruction in application, core dumped.
        ```

2. 深入理解 [trap.S](https://github.com/LearningOS/rust-based-os-comp2022/blob/main/os3-ref/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

    1. L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。

    * 解答：一种是开始执行 APP， 另一种场景是从 trap 中返回到 U 模式继续执行 APP，在这两种场景中， `a0` 都是指向要恢复的 `TrapContext` 的指针。

    2. L46-L51：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

        ```ASN.1
        ld t0, 32*8(sp)
        ld t1, 33*8(sp)
        ld t2, 2*8(sp)
        csrw sstatus, t0
        csrw sepc, t1
        csrw sscratch, t2
        ```


    * 解答：从内核栈顶的 Trap 上下文恢复通用寄存器和 CSR。
        - `sepc` 存储了Trap发生之前执行的最后一条指令的地址。
        - `sstatus` 记录Trap发生之前CPU处在哪个特权级。
        - `sscratch` 首先是保存了内核栈的地址，其次它可作为一个中转站让 `sp` （目前指向的用户栈的地址）的值可以暂时保存在 `sscratch` 。

    3. L53-L59：为何跳过了 `x2` 和 `x4`？

        ```ASN.1
        ld x1, 1*8(sp)
        ld x3, 3*8(sp)
        .set n, 5
        .rept 27
           LOAD_GP %n
           .set n, n+1
        .endr
        ```

    * 解答：要基于`x2`来找到每个寄存器应该被保存到的正确的位置。

        `x4`作为tp寄存器，一般不使用。

    4. L63：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

        ```ASN.1
        csrrw sp, sscratch, sp
        ```

    * 解答：sp指向用户栈栈顶，sscratch指向内核栈栈顶。

    5. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

    * 解答：发生在 `sret` 指令。因为我们在`csrrw sp, sscratch, sp`之后， `SPP` 就是 `U`, 所以会进入用户态。

    6. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

        ```ASN.1
        csrrw sp, sscratch, sp
        ```

    * 解答：sp指向内核栈栈顶，sscratch指向用户栈栈顶。

    7. 从 U 态进入 S 态是哪一条指令发生的？

    * 解答：`__alltraps`中的`csrrw sp, sscratch, sp`指令，从U态进入S态。