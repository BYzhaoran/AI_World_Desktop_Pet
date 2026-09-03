#[derive(Clone, Copy, Debug)]
pub struct WindowContext { pub hours_since_important: f32, pub important_today: u32, pub recent_event_count: u32, pub goal_pressure: f32, pub relationship_opportunity: f32, pub random_factor: f32 }

pub fn important_probability(context: WindowContext, target_per_day: f32) -> f32 {
    let daily_pressure = (target_per_day - context.important_today as f32) * 0.16;
    let elapsed_pressure = (context.hours_since_important / 8.0).clamp(0.0, 1.0) * 0.28;
    let context_score = (context.goal_pressure + context.relationship_opportunity).clamp(0.0, 2.0) * 0.16;
    let repetition_penalty = (context.recent_event_count as f32 / 10.0).clamp(0.0, 1.0) * 0.18;
    (0.08 + daily_pressure + elapsed_pressure + context_score + context.random_factor * 0.12 - repetition_penalty).clamp(0.02, 0.86)
}

pub fn should_schedule(context: WindowContext, target_per_day: f32) -> bool { important_probability(context, target_per_day) >= 0.5 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn daily_balance_reduces_probability() { let base=WindowContext{hours_since_important:7.0,important_today:0,recent_event_count:1,goal_pressure:0.5,relationship_opportunity:0.5,random_factor:0.5}; assert!(important_probability(base,2.0)>important_probability(WindowContext{important_today:2,..base},2.0)); }
    #[test] fn context_can_break_cooldown() { let ctx=WindowContext{hours_since_important:1.0,important_today:0,recent_event_count:0,goal_pressure:1.0,relationship_opportunity:1.0,random_factor:1.0}; assert!(should_schedule(ctx,2.0)); }
}
