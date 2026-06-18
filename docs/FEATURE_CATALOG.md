# gazeOff Feature Catalog

This catalog reflects the current application and is the source of truth for the product website.

## Break System

- Configurable work interval, short-break duration, long-break duration, and long-break frequency.
- Recommended defaults: 20 minutes of work, a 25-second short break, and a 3-minute long break every third cycle.
- Automatic long-break upgrade after a 90-minute uninterrupted session.
- Guaranteed cursor-following `5, 4, 3, 2, 1` countdown immediately before an automatic break.
- Typing-aware start behavior that waits for a natural input pause, with a bounded fallback.
- Skip and delay actions with one-minute and five-minute delay options.
- A completed break clears accumulated skip debt.

## Break Modes

- **Chill:** skipping starts a fresh full interval.
- **Smart:** skipped breaks return progressively sooner: five minutes, three minutes, then one minute.
- **Locked In:** breaks cannot be skipped or delayed.
- Smart mode requires a completed break after repeated skipping.
- Overtime nudges appear at 30, 45, 60, 75 minutes, and later milestones when break debt exists.

## Smart Pause

- Pauses work accumulation during detected meetings and calls using Windows microphone/camera activity.
- Pauses during fullscreen browser or video-player playback.
- Pauses while fullscreen games are active.
- Optional extended delay while typing or dragging.
- Configurable idle pause threshold.
- AFK return prompt asks whether the away period counted as a break.
- AFK timeout classifies an unanswered prompt as a completed away break.
- Optional sounds for smart-pause and return-from-idle events.

## Blink And Posture

- Independent blink and posture reminders with configurable intervals.
- Animated, background-free reminder marks: blinking eyes and a bent arrow that straightens.
- Optional “Blink” and “Fix posture” labels.
- Placement at corners, edges, center, bottom center, or near the cursor.
- Live previews from Studio without waiting for a timer.
- Optional screen dimming and reminder sounds.
- Separate reminder and nudge volume controls.
- Option to continue wellness reminders during meetings, videos, games, and manual pauses.

## Break Experience

- Time-based sky that changes with the hour.
- Windows wallpaper background with configurable live blur.
- Smooth wallpaper unblur and fade back to the desktop.
- Plain black break background that keeps the timer and controls visible.
- Calm message library plus optional playful phrases.
- Optional message-free timer view.
- Background-aware skip language.
- Matching start and end sound pairs: Original, Bell, Bubbles, Flute, Harp, Piano, Twinkle, and Whoosh.

## Recovery And Statistics

- Live eye-recovery score based on uninterrupted work, completed breaks, delays, and skips.
- Clear, Accumulating, and Due recovery states.
- Today view with completed breaks, skipped breaks, total break time, longest uninterrupted session, and day streak.
- Seven-day activity history.
- Session timeline distinguishing sessions that ended with a break from skipped sessions.
- Daily rollover and persisted streak tracking.

## Windows Experience

- Tray-first operation with a live state icon and tooltip.
- Tray actions for taking a break, pausing, resuming, testing reminders, opening Studio, and quitting.
- Launch-at-login and start-timer-on-launch controls.
- Acrylic light and dark Studio themes.
- Custom title bar with minimize, maximize or restore, and close behavior.
- Local JSON persistence with versioned settings migration and Reset to Defaults.
- Test Surfaces for countdown, overtime, AFK, blink, posture, and the break overlay.

## Product Principles

- Local-first and quiet by default.
- Interruptions remain brief, predictable, and configurable.
- Smart behavior adapts timing without silently discarding recovery needs.
- Visuals feel calm and expressive without hiding core controls.
