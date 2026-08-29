//! 贪吃蛇（终端版，纯 Rust + libc，无其他依赖）
//!
//! 操作说明：
//!   ↑/↓/←/→ 或 W A S D 或 H J K L : 控制方向
//!   P            : 暂停 / 继续
//!   R            : 重新开始
//!   Q / Esc      : 退出

use std::io::Write;
use std::time::Duration;

const MIN_BOARD_W: u16 = 20; // 棋盘最小宽度（格）
const MIN_BOARD_H: u16 = 10; // 棋盘最小高度（格）
const START_SPEED_MS: u64 = 200;
const MIN_SPEED_MS: u64 = 100;
const SPEEDUP_PER_FOOD: u64 = 2; // 每吃一个食物提速 2ms

// ANSI 转义（界面只使用 ASCII 字符，避免不同终端全角/半角差异导致的错位）
const HOME: &str = "\x1b[H";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";
const ALT_SCREEN_ON: &str = "\x1b[?1049h";
const ALT_SCREEN_OFF: &str = "\x1b[?1049l";
const RESET: &str = "\x1b[0m";
const SGR_HEAD: &str = "\x1b[1;32m";
const SGR_BODY: &str = "\x1b[38;5;28m";
const SGR_FOOD: &str = "\x1b[1;31m";

type Point = (u16, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn delta(self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    fn is_opposite(self, other: Dir) -> bool {
        self == other.reverse()
    }

    fn reverse(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    Up,
    Down,
    Left,
    Right,
    Pause,
    Restart,
    Quit,
    Eof,
    Other,
}

// 简单的确定性伪随机数（便于单元测试），避免引入 rand 依赖
#[derive(Debug)]
struct Fnv(u64);

impl Fnv {
    fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next_u64(&mut self) -> u64 {
        // SplitMix64 风格的混淆，质量足够做食物随机
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z ^= z >> 33;
        z = z.wrapping_mul(0xff51_afd7_ed55_8ccd);
        z ^= z >> 33;
        z = z.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        z ^= z >> 33;
        self.0 = z;
        self.0
    }
    fn range(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

/// 占用网格单元状态：0 空 / 1 蛇身 / 2 食物
const EMPTY: u8 = 0;
const SNAKE: u8 = 1;
const FOOD: u8 = 2;

#[derive(Debug)]
struct Game {
    width: u16,
    height: u16,
    snake: Vec<Point>, // snake[0] 是蛇头
    pending_dir: Dir,  // 本帧输入的方向（防止一帧内两次转向）
    food: Point,
    score: u32,
    over: bool,
    rng: Fnv,
    occ: Vec<u8>, // 占用网格，width*height，O(1) 查询某格是否被占
}

fn cell_index(width: u16, p: Point) -> usize {
    p.1 as usize * width as usize + p.0 as usize
}

impl Game {
    fn new(width: u16, height: u16, seed: u64) -> Self {
        let width = width.max(4);
        let height = height.max(3);
        let cy = height / 2;
        let cx = width / 2;
        let snake = vec![(cx, cy), (cx - 1, cy), (cx - 2, cy)];
        let mut g = Game {
            width,
            height,
            snake,
            pending_dir: Dir::Right,
            food: (0, 0),
            score: 0,
            over: false,
            rng: Fnv::new(seed),
            occ: vec![EMPTY; width as usize * height as usize],
        };
        for &p in &g.snake {
            g.occ[cell_index(width, p)] = SNAKE;
        }
        g.spawn_food();
        g
    }

    /// 依据占用网格收集所有空格，从中随机取一个放食物（O(w·h)，不再逐格 contains 蛇身）
    fn spawn_food(&mut self) {
        let free: Vec<usize> = (0..self.occ.len())
            .filter(|&i| self.occ[i] == EMPTY)
            .collect();
        if free.is_empty() {
            // 棋盘填满（理论上的胜利态），占位避免再次触发吃
            self.food = (0, 0);
            return;
        }
        let pick = self.rng.range(free.len() as u64) as usize;
        let i = free[pick];
        self.food = (
            (i % self.width as usize) as u16,
            (i / self.width as usize) as u16,
        );
        self.occ[i] = FOOD;
    }

    #[cfg(test)]
    fn rebuild_occ(&mut self) {
        let w = self.width;
        self.occ = vec![EMPTY; w as usize * self.height as usize];
        for &p in &self.snake {
            self.occ[cell_index(w, p)] = SNAKE;
        }
        self.occ[cell_index(w, self.food)] = FOOD;
    }

    /// 转向：忽略反向输入（避免 180° 掉头撞自己）
    fn turn(&mut self, d: Dir) {
        if !d.is_opposite(self.pending_dir) {
            self.pending_dir = d;
        }
    }

    /// 执行一步，返回是否吃到食物
    fn step(&mut self) -> bool {
        if self.over {
            return false;
        }
        let dir = self.pending_dir;
        let (dx, dy) = dir.delta();
        let (hx, hy) = self.snake[0];
        let nx = hx as i32 + dx;
        let ny = hy as i32 + dy;

        // 撞墙
        if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
            self.over = true;
            return false;
        }

        let head = (nx as u16, ny as u16);
        let head_idx = cell_index(self.width, head);
        let eating = head == self.food;

        if !eating {
            // 尾巴本帧会让开：先把它从占用网格清掉，再判断头是否撞到"仍被占的身体"
            let tail = *self.snake.last().unwrap();
            let tail_idx = cell_index(self.width, tail);
            self.occ[tail_idx] = EMPTY;
            if self.occ[head_idx] == SNAKE {
                // 撞身体死亡，恢复尾巴占用以保持状态一致
                self.occ[tail_idx] = SNAKE;
                self.over = true;
                return false;
            }
        }

        // 移动：新头占用网格置 1，尾部在下方弹出（非进食时）
        self.snake.insert(0, head);
        self.occ[head_idx] = SNAKE;

        if eating {
            self.spawn_food();
            self.score += 1;
            true
        } else {
            self.snake.pop();
            false
        }
    }

    /// 当前每步耗时（毫秒），分数越高越快（有下限）
    fn speed_ms(&self) -> u64 {
        std::cmp::max(
            MIN_SPEED_MS,
            START_SPEED_MS.saturating_sub(self.score as u64 * SPEEDUP_PER_FOOD),
        )
    }
}

// ---------- 终端 (libc) ----------

struct RawTerm {
    orig: libc::termios,
}

impl RawTerm {
    fn enable() -> std::io::Result<Self> {
        let mut orig: libc::termios = unsafe { std::mem::zeroed() };
        if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut orig) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        let mut raw = orig;
        unsafe {
            libc::cfmakeraw(&mut raw);
            raw.c_cc[libc::VMIN] = 0; // 非阻塞读
            raw.c_cc[libc::VTIME] = 0;
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &raw);
        }
        Ok(RawTerm { orig })
    }
}

impl Drop for RawTerm {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.orig);
        }
    }
}

/// 非阻塞地读取一个按键（支持方向键的 ESC [ X 序列）
/// `is_tty` 预先算好，避免每次按键都调用 isatty 系统调用
fn read_key(is_tty: bool) -> Option<Key> {
    let mut buf = [0u8; 1];
    let n = unsafe { libc::read(libc::STDIN_FILENO, buf.as_mut_ptr() as *mut _, 1) };
    if n < 0 {
        return None; // EAGAIN：当前没有按键
    }
    if n == 0 {
        // TTY 上 VMIN=0/VTIME=0 时空读也返回 0，所以只有非 TTY（管道 EOF）才是真正结束
        if is_tty {
            return None;
        }
        return Some(Key::Eof);
    }
    match buf[0] {
        b'w' | b'W' | b'k' | b'K' => Some(Key::Up),
        b's' | b'S' | b'j' | b'J' => Some(Key::Down),
        b'a' | b'A' | b'h' | b'H' => Some(Key::Left),
        b'd' | b'D' | b'l' | b'L' => Some(Key::Right),
        b'p' | b'P' => Some(Key::Pause),
        b'r' | b'R' => Some(Key::Restart),
        b'q' | b'Q' | 0x03 => Some(Key::Quit), // 0x03 = Ctrl+C
        0x1b => {
            // 可能单独按了 Esc，也可能是方向键序列 ESC [ A/B/C/D
            let mut b2 = [0u8; 1];
            let n2 = unsafe { libc::read(libc::STDIN_FILENO, b2.as_mut_ptr() as *mut _, 1) };
            if n2 <= 0 {
                Some(Key::Quit)
            } else if b2[0] == b'[' || b2[0] == b'O' {
                let n3 = unsafe { libc::read(libc::STDIN_FILENO, b2.as_mut_ptr() as *mut _, 1) };
                if n3 > 0 {
                    match b2[0] {
                        b'A' => Some(Key::Up),
                        b'B' => Some(Key::Down),
                        b'C' => Some(Key::Right),
                        b'D' => Some(Key::Left),
                        _ => Some(Key::Other),
                    }
                } else {
                    Some(Key::Other)
                }
            } else {
                Some(Key::Other)
            }
        }
        _ => Some(Key::Other),
    }
}

// ---------- 渲染 ----------

fn is_stdin_tty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) == 1 }
}

/// 在指定的棋盘格位置写内容（p 为 0 基格子坐标，每格宽 2 列）
fn put(f: &mut String, p: Point, text: &str, sgr: &str) {
    f.push_str(&format!("\x1b[{};{}H", p.1 as usize + 1, p.0 as usize * 2 + 1));
    if !sgr.is_empty() {
        f.push_str(sgr);
    }
    f.push_str(text);
    if !sgr.is_empty() {
        f.push_str(RESET);
    }
}

/// 画棋盘边框（仅开局画一次，纯 ASCII）
fn render_board(game: &Game) {
    let (bw, bh) = (game.width, game.height);
    let mut f = String::with_capacity(4096);
    let hwall = "-".repeat(bw as usize * 2);
    f.push_str(HOME);
    f.push_str(&format!("+{}+\n", hwall));
    for _ in 0..bh {
        f.push_str(&format!("|{}|\n", " ".repeat(bw as usize * 2)));
    }
    f.push_str(&format!("+{}+", hwall));
    print!("{f}");
    let _ = std::io::stdout().flush();
}

/// 差分重绘（不清屏，避免闪烁）：
/// 1) 擦掉"不再被占用"的旧格子（用占用网格 O(1) 判断）  2) 画当前的蛇与食物  3) 更新状态栏
fn render_frame(game: &Game, prev_snake: &[Point], prev_food: Point, hi_score: u32, paused: bool) {
    let mut f = String::with_capacity(1024);

    // 1) 擦除移动的旧格子（蛇尾、旧食物等）——占用网格为 0 即已空出
    for &p in prev_snake {
        if game.occ[cell_index(game.width, p)] == EMPTY {
            put(&mut f, p, "  ", "");
        }
    }
    if game.occ[cell_index(game.width, prev_food)] == EMPTY {
        put(&mut f, prev_food, "  ", "");
    }

    // 2) 画当前的蛇与食物
    let head = game.snake[0];
    for p in &game.snake {
        if *p == head {
            put(&mut f, *p, "OO", SGR_HEAD);
        } else {
            put(&mut f, *p, "oo", SGR_BODY);
        }
    }
    put(&mut f, game.food, "**", SGR_FOOD);

    // 3) 状态栏（固定位置整行重写，重要信息放前面；帮助文字按宽度自适应避免溢出）
    let tag = if paused {
        " [PAUSED]"
    } else if game.over {
        " [GAME OVER]"
    } else {
        ""
    };
    let core = format!(
        "{}  score: {:<3} best: {:<3} len: {:<3}",
        tag,
        game.score,
        hi_score,
        game.snake.len()
    );
    let width = game.width as usize * 2;
    let line = if width >= core.chars().count() + 48 {
        format!("{}  WASD/arrows/hjkl  P pause  R restart  Q quit", core)
    } else {
        format!("{}  WASD/hjkl P/R/Q", core)
    };
    f.push_str(&format!("\x1b[{};1H", game.height as usize + 2));
    f.push_str(&format!("{:<width$}", line, width = width));

    print!("{f}");
    let _ = std::io::stdout().flush();
}

// ---------- 主流程 ----------

fn seed_from_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(42)
}

/// 获取终端尺寸（列, 行），失败时退回 80x24
fn terminal_size() -> (u16, u16) {
    // 注意字段顺序必须与 libc 的 winsize 一致：ws_row, ws_col, ws_xpixel, ws_ypixel
    #[repr(C)]
    #[derive(Default)]
    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }
    let mut ws = Winsize::default();
    if unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ as libc::c_ulong, &mut ws) }
        == 0
        && ws.ws_col >= 24
        && ws.ws_row >= 8
    {
        (ws.ws_col, ws.ws_row)
    } else {
        (80, 24)
    }
}

/// 根据终端列/行计算棋盘格数（每格 2 列 × 1 行，另需边框和状态栏）
fn board_size_from_terminal(cols: u16, rows: u16) -> (u16, u16) {
    let w = (cols.saturating_sub(2) / 2).max(MIN_BOARD_W);
    let h = rows.saturating_sub(4).max(MIN_BOARD_H);
    (w, h)
}

fn main() {
    let term = match RawTerm::enable() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("无法切换到 raw 终端模式（需要在一个真实的终端里运行）: {e}");
            std::process::exit(1);
        }
    };
    let is_tty = is_stdin_tty();
    let mut hi_score = 0u32;
    let (cols, rows) = terminal_size();
    let board = board_size_from_terminal(cols, rows);
    let mut game = Game::new(board.0, board.1, seed_from_time());
    let mut paused = false;

    print!("{ALT_SCREEN_ON}{HIDE_CURSOR}");
    render_board(&game);
    let mut prev_snake: Vec<Point> = game.snake.clone();
    let mut prev_food = game.food;
    let mut needs_render = true;

    loop {
        // 仅在状态有变化或收到按键时才重绘，避免空转时重复刷屏
        if needs_render {
            render_frame(&game, &prev_snake, prev_food, hi_score, paused);
            prev_snake = game.snake.clone();
            prev_food = game.food;
            needs_render = false;
        }

        // 收键
        if let Some(k) = read_key(is_tty) {
            match k {
                Key::Up if !paused => game.turn(Dir::Up),
                Key::Down if !paused => game.turn(Dir::Down),
                Key::Left if !paused => game.turn(Dir::Left),
                Key::Right if !paused => game.turn(Dir::Right),
                Key::Pause => paused = !paused,
                Key::Restart => {
                    hi_score = hi_score.max(game.score);
                    game = Game::new(game.width, game.height, seed_from_time());
                    paused = false;
                }
                Key::Eof => break, // 非交互环境（如 stdin 管道关闭）时安全退出
                Key::Quit => break,
                _ => {}
            }
            needs_render = true;
            continue; // 处理完按键立刻循环（排空缓冲中的其他按键）
        }

        if game.over || paused {
            // 结束后/P 暂停时：等 R/Q 在上面已处理，这里只休眠避免空转
            std::thread::sleep(Duration::from_millis(10));
            continue;
        }

        // 到达一帧时间点 → 走一步
        std::thread::sleep(Duration::from_millis(game.speed_ms()));
        game.step();
        needs_render = true;
    }

    hi_score = hi_score.max(game.score);
    print!("{ALT_SCREEN_OFF}{SHOW_CURSOR}");
    eprintln!("\n最终最高分: {hi_score}，感谢游玩！");
    drop(term);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game(seed: u64) -> Game {
        Game::new(40, 18, seed)
    }
    #[test]
    fn snake_moves_right_initially() {
        let mut g = game(1);
        let head = g.snake[0];
        g.step();
        assert_eq!(g.snake[0].0, head.0 + 1);
        assert!(!g.over);
    }

    #[test]
    fn reverse_turn_is_ignored() {
        let mut g = game(1);
        let head = g.snake[0];
        g.turn(Dir::Left); // 初始向右，向左应被忽略
        g.step();
        assert_eq!(g.snake[0].0, head.0 + 1);
        assert!(!g.over);
    }

    #[test]
    fn valid_turn_applied_next_step() {
        let mut g = game(1);
        g.turn(Dir::Up);
        let head = g.snake[0];
        g.step();
        assert_eq!(g.snake[0].1, head.1 - 1);
    }

    #[test]
    fn last_key_in_frame_wins() {
        // 同帧内连续两次转向，只有最后一次生效（记录在 pending_dir）
        let mut g = game(1);
        g.turn(Dir::Up);
        assert_eq!(g.pending_dir, Dir::Up);
        g.turn(Dir::Left); // 相对 Up 不是反向，应覆盖 Up
        assert_eq!(g.pending_dir, Dir::Left);
    }

    #[test]
    fn eating_food_grows_snake_and_scores() {
        let mut g = game(1);
        g.food = (g.snake[0].0 + 1, g.snake[0].1);
        g.rebuild_occ();
        let len = g.snake.len();
        let ate = g.step();
        assert!(ate);
        assert_eq!(g.snake.len(), len + 1);
        assert_eq!(g.score, 1);
        assert!(!g.snake.contains(&g.food)); // 新食物不在蛇身上
    }

    #[test]
    fn hitting_wall_ends_game() {
        let mut g = game(1);
        g.snake = vec![(g.width - 1, 5), (g.width - 2, 5), (g.width - 3, 5)];
        g.rebuild_occ();
        g.pending_dir = Dir::Right;
        g.food = (0, 0);
        g.step();
        assert!(g.over);
    }

    #[test]
    fn hitting_self_ends_game() {
        let mut g = game(1);
        // 围成圈，向下会撞到自己身体 (10,6)
        g.snake = vec![(10, 5), (11, 5), (11, 6), (10, 6), (9, 6), (9, 5), (9, 4), (10, 4)];
        g.rebuild_occ();
        g.turn(Dir::Down);
        g.food = (0, 0);
        g.step();
        assert!(g.over);
    }

    #[test]
    fn moving_into_own_tail_is_allowed() {
        // 尾巴会让开：头撞向当前位置的尾巴不算死
        let mut g = game(1);
        // 蛇链: (10,5)头 → (9,5) → (9,6) → (10,6)尾，向下一步 (10,6) 正是尾巴
        g.snake = vec![(10, 5), (9, 5), (9, 6), (10, 6)];
        g.rebuild_occ();
        g.turn(Dir::Down);
        g.food = (0, 0);
        g.step();
        assert!(!g.over);
        assert_eq!(g.snake[0], (10, 6));
    }

    #[test]
    fn speed_increases_with_score_but_has_floor() {
        let mut g = game(1);
        let base = g.speed_ms();
        g.score = 100;
        assert!(g.speed_ms() < base);
        g.score = 10_000_000;
        assert_eq!(g.speed_ms(), MIN_SPEED_MS);
    }

    #[test]
    fn food_never_spawns_on_snake() {
        let mut g = game(42);
        for _ in 0..100 {
            assert!(!g.snake.contains(&g.food));
            g.spawn_food();
            assert!(!g.snake.contains(&g.food));
        }
    }
}
