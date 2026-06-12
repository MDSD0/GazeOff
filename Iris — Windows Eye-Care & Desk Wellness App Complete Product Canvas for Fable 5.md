# Iris — Windows Eye-Care & Desk Wellness App
## Complete Product Canvas for Fable 5

***

## 1. Product Summary

This document is the strategic and creative north star for a new Windows-first tray application in the screen-wellness category. The app is a lightweight, adaptive, and beautifully minimal recovery companion for desk workers — one that lives quietly in the system tray, understands context, and surfaces break nudges at the right moment rather than every twenty minutes like clockwork.

The product is not a grandpa-serious compliance tool and not a cheap gamified timer. It occupies a third position: a premium, warm, modern wellness utility that respects the user's intelligence, trusts their autonomy, and — with an optional toggle — can be lightly witty and internet-native without ever becoming cringe. It should feel like something a designer built for themselves, not something a corporate IT policy mandated.

The app is inspired by LookAway and DeskRest but builds beyond both — with better adaptive escalation logic, a more honest recovery-state model, and a cleaner creative voice.

***

## 2. Discussion Summary & Design Intent

This product emerged from a research conversation with the developer, Muhammad Saadullah, and reflects the following explicitly stated intentions:

**Technology:** Windows-first. Tray-first. Lightweight. Tauri + Rust backend with a minimal frontend. Resource-efficient, fast at startup, low idle memory. No Electron.

**Tone:** Two-layer copy system. Default tone is calm, premium, human. Optional playful mode is Gen Z / internet-native (e.g., "touch grass twin") — available via toggle, off by default.

**Feature philosophy:** Everything significant is optional and toggleable. The default profile should be good enough out of the box that most users never open settings. Sensible defaults matter as much as configurability.

**Break model:** Not a dumb interval timer. The app should behave as an adaptive recovery companion — understanding sessions, context, skip debt, inactivity, focus state, and engagement level before deciding when and how firmly to surface a reminder.

**Design intent:** Minimal, frosted, foggy, ambient, transparent. Glass-like surfaces. Calm. Premium. The visual feeling of morning light through frosted glass — not neon dashboards, not achievement badges, not colorful health-app gradients.

**Enforcement:** Multiple strictness modes. A lenient mode using color state and soft nudges only. A smart break mode with escalating enforcement that can — if the user opts in — trigger a forced unskippable break after repeated ignores.

**Health honesty:** The app should surface evidence-informed rationale without claiming to be a medical device. Information about why breaks matter should be available, concise, and trustworthy, not medicalized or alarmist.

***

## 3. Scientific Research Summary

### 3.1 Digital Eye Strain (DES) and Computer Vision Syndrome (CVS)

Digital eye strain — also called Computer Vision Syndrome — is well-established as a consequence of prolonged screen use. Its symptoms include dry eyes, blurred vision, eye soreness, headaches, burning sensation, and difficulty refocusing. CVS prevalence in populations with significant screen exposure ranges from roughly 56% to 90% across studies, and multiple reviews estimate that 70-80% of adults who use digital devices for at least two hours daily present with CVS symptoms. A 2025 study found 78.7% of young adults experienced eye strain symptoms, and 68.1% reported headaches or discomfort during or after screen use.

CVS operates through several mechanisms. The ocular surface mechanism is most relevant to this product: prolonged near-focus reduces blink rate and increases incomplete blinking, which disrupts tear film stability, allows greater tear evaporation, and progressively dries the ocular surface. An accommodative mechanism also operates — the ciliary muscles required to maintain near focus fatigue during sustained near-work, a process called near-work-induced transient myopia (NITM), which takes time to dissipate after the task ends.

### 3.2 Blink Rate Reduction

Blink rate during screen use is substantially and consistently reduced. Resting blink rate is typically 14-16 blinks per minute, but during active screen use this falls to 4-6 blinks per minute. One study recorded a drop from 18.4 to 3.6 blinks per minute; another from 22 to 7. The research also shows that the more significant problem may not be blink rate per se, but *incomplete blinks* — where the upper eyelid does not fully traverse the corneal surface. Incomplete blinks fail to distribute meibomian gland lipids across the tear film, leading to evaporative dry eye even when overall blink count appears adequate. During video games and active tasks, studies found that 88-92% of blinks were incomplete.

The practical implication for this app: a blink reminder is not a gimmick. Prompting full, slow, complete blinks at intervals is biologically meaningful — more so than simply reminding users to look away.

### 3.3 The 20-20-20 Rule: Useful Framing, Limited Evidence

The 20-20-20 rule (look at something 20 feet away for 20 seconds every 20 minutes) is widely cited but its evidence base is more mixed than its ubiquity suggests. One well-powered 2022 study found that 20-second scheduled breaks showed no significant effect on reported symptoms, reading speed, or task accuracy. The study authors were explicit that this does not mean breaks are unhelpful — rather, the *duration* may be insufficient. Animal model data suggests that sustained breaks of five or more minutes every hour may be needed to negate myopiagenic effects, compared to 20-second micro-breaks.

A different 2025 study evaluating multiple break schedules during a 40-minute reading task found that both frequent (3-break) and self-paced break conditions showed significantly less eye irritation, eye strain, and NITM compared to the no-break condition — and that accommodative variability was better in self-paced break conditions. A Cochrane-adjacent review-level study found the 20-20-20 rule effective at reducing dry eye symptoms and DES over 2 weeks of reminder use, though improvements did not persist after reminders stopped. NIOSH research found that adding four short 5-minute supplementary breaks throughout the workday reduced eye soreness, visual blurring, and upper-body discomfort compared to conventional break schedules, without reducing productivity.

**Summary for product framing:** The evidence supports *frequent breaks with meaningful duration* more strongly than a rigid 20-second rule. Self-paced or adaptive breaks have emerging evidence as potentially superior to fixed-interval scheduling. The app should frame itself as adaptive and evidence-informed, not as a "20-20-20 app." The 20-20-20 rule can be offered as a starting template — not as the product identity.

### 3.4 Posture and Musculoskeletal Overlap

Eye strain and musculoskeletal discomfort do not occur independently. Studies consistently document strong co-occurrence of neck, shoulder, and upper back pain alongside CVS symptoms among screen users. A 2025 study found neck pain at 73.8%, headaches at 70.6%, and eye strain at 64.4% — all co-occurring in screen-heavy populations. A Swedish ergonomics study found that oculomotor fatigue may induce secondary changes in postural muscle innervation in the neck-scapular area, creating a bidirectional reinforcement loop between eye and neck symptoms. A 2025 paper found combined digital eye strain and musculoskeletal disorders at 63.8% prevalence. Screen setup, viewing angle, and workstation ergonomics modulate both eye strain and neck/shoulder load simultaneously.

**Product implication:** Posture reminders are not a peripheral nice-to-have. They are scientifically co-motivated with eye break reminders, and the combination addresses the real pattern of screen worker symptoms more completely than eye-only reminders.

### 3.5 Validated Symptom Tools

Several validated questionnaires exist for measuring CVS. The most comprehensive is the **CVSS17** (Computer Vision Symptom Scale, 17 items), which covers both internal symptoms (visual, accommodative) and external symptoms (ocular surface, dry eye), with well-established reliability and discriminatory power across five severity levels. The **CVS-Q** (16 symptoms, sensitivity and specificity above 70%) is a parallel validated tool. The **OSDI** (Ocular Surface Disease Index) is used for dry eye specifically.

**Product claim boundary:** The app should describe itself as supporting habit change informed by research on eye strain and desk recovery. It should not use language that implies clinical diagnosis, treatment, or prevention of disease. Symptom language (dry eyes, eye soreness, headaches, blurred vision) is appropriate in informational tooltips but should be presented as common screen-use complaints addressed by habit, not medical symptoms requiring clinical intervention.

***

## 4. Competitor Feature Map

### 4.1 LookAway — Complete Feature Catalog

LookAway is a macOS app built around smart, context-aware breaks with an increasingly complete wellness layer.

#### Break Configuration
- **Short breaks** with customizable screen time interval and break duration.
- **Long breaks** triggered after a set number of short breaks (e.g., every 2, 3, or 4 short breaks); independently configurable duration.
- **Preset modes** — Balanced (20 min / 20 sec), Deep Focus (45 min / 30 sec + 5 min long break every 3 short), Eye Care (15 min / 15 sec), Wellness (25 min / 45 sec + 5 min long break every 2 short).
- **Skip difficulty (3 levels):** Casual (skip anytime), Balanced (skip button disabled for a few seconds then enabled), Hardcore (cannot skip at all — before or during the break).
- **"Don't show while typing or dragging"** — break is postponed until user finishes mid-sentence or file drag. The most-requested feature in LookAway 1.10.
- **"Let me End Break early if nearly done"** — once enough of the break has elapsed, the Skip button converts to an End Break button.
- **Lock Mac automatically when break starts** — optional extra enforcement for stepping away.

#### Pre-Break Reminders & Countdown
- **Pre-break notification toggle** — advance heads-up before the break appears.
- **Reminder timing** — how far in advance the reminder appears; default 1 minute.
- **Reminder visibility duration** — how long the notification stays on screen before disappearing; default 10 seconds; now configurable.
- **Break countdown** — visual timer showing when the next break begins.
- **Overtime nudge** — if the user keeps working past their planned session length, LookAway surfaces an additional nudge beyond the normal reminder cycle.

#### Wellness Reminders
- **Blink reminders** — interval-based nudges to blink at healthy intervals. Subtle, non-intrusive animations. This is a LookAway differentiator that DeskRest does not explicitly offer.
- **Posture reminders** — nudges to correct posture mid-session. Added in v1.10.

#### Smart Pause
- **Meetings & calls** — detects active microphone and/or camera usage; postpones breaks during active calls. Configurable per device (e.g., exclude external mics). Works with Zoom, Meet, Teams, and others. (LookAway also now treats *dictation* like typing, pausing breaks accordingly.)
- **Video playback** — pauses breaks during video. Configurable as "frontmost only" or "running in background too." Plex, YouTube, VLC, Netflix, QuickTime all supported.
- **Deep Focus Apps** — user-defined app list; pauses when foreground, foreground+fullscreen, or any open state.
- **Fullscreen gaming** — automatic detection; no manual configuration needed.
- **Screen recording & sharing** — detects OBS, QuickTime recording, Zoom screen share, etc. Prevents reminders from appearing in recordings.
- **Cooldown after smart pause ends** — configurable delay after a high-engagement activity ends before a pending break is shown (e.g., 1, 2, or 5 minutes). Allows the user a moment to re-establish focus before being interrupted.
- **Focus Filters** — integrates with macOS Focus modes; automatically pauses when a specified Focus mode is active.
- **Idle time detection** — automatically pauses or resets when user steps away from desk.

#### Custom Messages
- Custom break screen messages, short and long separately.
- Any language supported.
- Random selection from user list each break.
- Example built-in lines include "Blink and breathe," "Stretch your shoulders," "Take a deep breath and relax," "Eyes to the horizon," "You've earned this rest," and "Give your eyes a holiday."

#### Stats & Screen Score
- **Total screen time** for the day.
- **Break count** — how many breaks were taken.
- **Longest session** without a proper break.
- **Median session length** for a sense of normal rhythm.
- **App usage** during active screen sessions.
- **Website usage** by domain in supported browsers (Safari, Chrome, Edge, Firefox family; enabled per-browser in settings).
- **Screen Score** — starts at 100; decreases when breaks are skipped or sessions run too long; rewards shorter sessions and consistent break adherence. Composed of Session Discipline (up to 60 points) and Break Adherence (up to 40 points).

#### Scheduling / Office Hours
- Active days selection (e.g., Monday–Friday only).
- Active hours window (e.g., 9 AM–6 PM).

#### Automations & Scripting
- AppleScript triggers on break start and break end.
- Apple Shortcuts triggers on break start and break end.
- Example automations: dim screen, pause Spotify, change Slack status to away, enable Do Not Disturb, lock Mac on long break.

#### Other Notable
- Menu bar icon with live status.
- Multiple screen support.
- Gentle end-of-break chime.
- Option to hide menu bar icon entirely.
- Easter eggs in the UI.
- Eye Strain Risk Calculator on website (7 questions, behavioral risk score).
- iPhone/iPad break sync via LookAway Mirror — syncs Mac breaks to other Apple devices to prevent "break as scroll session."
- LookAway 2.0 introduced Liquid Glass redesign.

***

### 4.2 DeskRest — Complete Feature Catalog

DeskRest is a macOS app with a strong exercise-and-posture layer, cursor timer, and workday-boundary features.

#### Break Configuration
- **Short breaks** (e.g., 20 seconds) for eye relief.
- **Long breaks** (e.g., 3 minutes) for physical recovery; fully configurable.
- Configurable focus session lengths with automatic transition into break.
- **Skip and delay functions** — user has control; can configure strictness by hiding Skip/Delay buttons when strict mode is needed.
- **Blocks Cmd+Q during full-screen breaks** to prevent accidental break dismissal.

#### Cursor Timer
- **Cursor break reminder** — subtle countdown display appearing right next to the cursor, showing time remaining before next break.
- Introduced in DeskRest 1.7.0. Described as a "gentle productivity coach at your fingertips."
- Keeps users informed without requiring them to look at a menu bar or notification area.

#### Posture Reminders
- **Posture flash reminders** — brief, subtle screen flashes that appear without stopping work, reminding users to check their sitting position.
- Configurable and separate from break reminders.

#### Custom Exercises
- Users can **create unlimited custom break routines** targeting specific needs: eye exercises, shoulder rolls, standing transitions, etc.
- Exercises can be assigned differently to short vs. long breaks, preventing monotony.

#### Smart Pause & Idle Detection
- **Intelligent pause detection** for meetings, YouTube/video content, Focus modes, and inactivity.
- **Idle time settings** — timer pauses after N minutes of inactivity (default: 1 minute) and resets after M minutes of absence (default: 5 minutes). Both values are configurable.
- **Automatic detection of extended absences** — prevents break bombardment on return.

#### Notifications
- **Break notifications** — configurable timing, display duration, and visibility.
- **Pre-break advance warning** — users control timing and whether to show.
- Choose cursor timer, traditional notifications, or both simultaneously.
- Skip and Delay buttons can be hidden for strict discipline mode.

#### Working Hours / Clock Out
- **Set working hours** — configure active days and hours.
- **Clock Out Lock** — defines fixed end-of-workday; sends personalized notification to step away; creates a hard boundary between work and personal time.

#### Stats & Streaks
- **Streak tracking** — motivating display of break adherence streaks.
- **Activity calendar** — visual history of break consistency.
- Break completion patterns, skip rates, progress over time.

#### Shortcuts & Automation
- Keyboard shortcuts for triggering breaks, extending focus, restarting timers.
- Apple Shortcuts and Siri controls.

#### Other Notable
- Menu bar countdown with live remaining work time.
- Themes for break screens.
- Audio cues.

***

### 4.3 Comparative Feature Matrix

| Feature | LookAway | DeskRest | Gap / Opportunity |
|---|---|---|---|
| Short breaks | ✅ Configurable | ✅ Configurable | Parity |
| Long breaks | ✅ N-short-break trigger | ✅ Configurable | Parity |
| Pre-break warning | ✅ Timing configurable | ✅ Present | Parity |
| Skip difficulty levels | ✅ 3 levels (Casual / Balanced / Hardcore) | ✅ Hide skip/delay buttons | LookAway more granular |
| Smart pause: meetings | ✅ Camera + mic detection | ✅ Meeting detection | Parity |
| Smart pause: video | ✅ Frontmost or background | ✅ Video content | Parity |
| Smart pause: fullscreen games | ✅ Automatic | Not specified | LookAway advantage |
| Smart pause: screen recording | ✅ Yes | Not specified | LookAway advantage |
| Cooldown after activity ends | ✅ Configurable delay | Not specified | LookAway advantage |
| Pause-on-typing | ✅ Most-requested; added v1.10 | Not specified | LookAway advantage |
| Blink reminders | ✅ Dedicated interval-based | ❌ Eye rest only via breaks | LookAway advantage |
| Posture reminders | ✅ Yes | ✅ Flash reminders + exercises | DeskRest more visual/physical |
| Custom exercises | ❌ No | ✅ Unlimited custom routines | DeskRest advantage |
| Cursor-adjacent countdown | ❌ Floating countdown only | ✅ Cursor timer | DeskRest advantage |
| Stats / usage tracking | ✅ App + website + Screen Score | ✅ Streaks + calendar | LookAway deeper insight |
| Screen Score / health score | ✅ 0–100, session + adherence | ❌ Not equivalent | LookAway advantage |
| Streak tracking | ❌ Not listed | ✅ Yes | DeskRest advantage |
| Clock Out / end-of-day | ❌ Office hours only | ✅ Clock Out Lock | DeskRest advantage |
| Working hours schedule | ✅ Days + hours | ✅ Days + hours | Parity |
| Automations / scripting | ✅ AppleScript + Shortcuts | ✅ Keyboard shortcuts + Shortcuts | Parity |
| Idle time detection | ✅ Yes | ✅ Configurable pause + reset | Parity |
| Custom break messages | ✅ Per-break type, multi-language | ✅ Themes + audio | LookAway more textual |
| Forced unskippable breaks | ✅ Hardcore mode | ✅ Hidden skip buttons | Parity with different UX |
| Break debt / escalation model | ❌ Not dynamic | ❌ Not dynamic | **Major opportunity** |
| Adaptive recovery scoring | ✅ Screen Score (static formula) | ❌ Streak only | **Opportunity for improvement** |
| Display-off / black-screen breaks | ❌ Not listed | ❌ Not listed | **Unique opportunity** |
| Evidence-based tooltips / rationale | ❌ Blog only | ❌ Not listed | **Unique opportunity** |
| Playful / Gen Z copy mode | ❌ Neutral premium | ❌ Neutral premium | **Unique opportunity** |
| Windows-native | ❌ macOS only | ❌ macOS only | **Platform gap** |

***

## 5. Subtle Interaction Catalog

These are the small, precise, often undocumented behaviors that separate category-defining apps from basic timers. Each has design logic behind it.

### 5.1 The Pre-Break Heads-Up (not the break itself)
Both LookAway and DeskRest show a pre-break warning before the actual break begins. The logic: an abrupt interrupt feels coercive; a 30-60 second heads-up allows the user to reach a natural stopping point — finishing a sentence, a thought, a code line — before stepping away. This dramatically reduces resentment toward the app. The warning should dissolve after its display window without requiring dismissal, so it never itself becomes a friction point.

### 5.2 Cursor-Adjacent Countdown (DeskRest)
Rather than forcing the user to glance at a menu bar or check a widget, DeskRest shows a small countdown where the user's eyes already are — at the cursor. This is passive peripheral awareness, not an interruption. The logic: the user never has to remember where to look for the time. The countdown follows their attention. This is one of the most elegant ambient awareness features in the category and should be considered for this product.

### 5.3 Typing-Aware Break Delay (LookAway)
If a break fires while the user is typing mid-sentence, LookAway holds the break until typing stops. The logic: a break interrupting a sentence forces context loss and creates frustration. Waiting two seconds for a natural pause costs nothing but eliminates a consistent source of annoyance. This was LookAway's most-requested feature — a signal that users find mid-sentence interrupts disruptive enough to ask for this specifically.

### 5.4 Cooldown After Smart Pause Ends (LookAway)
When a meeting ends or a video stops, the user does not want their first moment of re-engagement to be a break reminder. LookAway inserts a configurable cooldown buffer (e.g., 1-5 minutes) after a smart pause ends before a pending break is shown. The logic: re-entry into focus is a cognitively active moment; interrupting it immediately would undermine the smart pause feature entirely.

### 5.5 End Break Early (LookAway)
Once a sufficient portion of the break has elapsed, the Skip button converts to "End Break" — signaling that the user has earned an early exit without the framing of "skipping." This is subtle but important: "End Break" communicates that enough rest happened; "Skip" communicates impatience and guilt.

### 5.6 Blocking App Quit During Break (DeskRest)
DeskRest blocks Cmd+Q during full-screen breaks to prevent accidental dismissal. The logic: a break that can be ended by a misclick is not a break. The full-screen moment should feel protected.

### 5.7 Skip Button Delay (LookAway Balanced mode)
The skip button is disabled for a few seconds before becoming clickable. The logic: a brief forced pause before skipping creates a tiny moment of pause-and-reconsider. Many users will sit through that two seconds and take the break rather than wait for the button — a behavioral nudge that preserves autonomy while reducing reflexive dismissal.

### 5.8 Idle Detection with Two Thresholds (DeskRest)
DeskRest uses two configurable thresholds: pause the timer after N minutes of inactivity (short absence), and reset it entirely after M minutes (extended absence). The logic: a 3-minute coffee break should not count as a recovery break. A 30-minute lunch should not resume with an immediate break demand on return. Two thresholds solve both cases.

### 5.9 Break Debt Escalation (Opportunity)
Neither LookAway nor DeskRest currently implements *dynamic break debt tracking* — a system where a skipped break adds to an escalating recovery obligation that changes the intensity or visibility of subsequent reminders. This is a meaningful product gap. The logic: one skipped break is a reasonable choice; five consecutive skipped breaks across a three-hour session is a different physiological situation that should be communicated differently.

### 5.10 Screen-Off / Black-Screen Breaks (Opportunity)
Neither competitor offers a genuine display-off or forced-black-screen mode as a break type. An optional black-screen break — dimming or blanking the display and encouraging the user to close their eyes, stand, or look away entirely — would serve both the energy-saving and the eye-rest use case more completely than a blurred overlay. It also communicates "step away from your screen" rather than "stare at a different thing on your screen."

***

## 6. Opportunity & Gap Analysis

### 6.1 Adaptive Recovery Scoring
LookAway's Screen Score is a step in the right direction — a single daily metric that reflects break adherence and session discipline. But it is still a post-hoc summary, not a live state indicator. An *in-session recovery state* — something the user can sense at a glance while working, not only at day's end — would be more behaviorally actionable. It should communicate something like "you're accumulating strain" or "you're in good shape" as a live ambient signal, not a daily report card.

### 6.2 Break Debt and Escalation Logic
Both competitors offer enforcement levels (skip difficulty) but neither dynamically escalates based on skip history within a session. A skip-debt model — where the first skip triggers a soft color shift, the second adds a warning, and the third (if in strict mode) triggers an unskippable break — would make the enforcement system feel principled and contextually intelligent rather than mechanically rigid.

### 6.3 Evidence Microcopy
Neither LookAway nor DeskRest surfaces in-app, per-feature rationale explaining *why* each setting matters. Users are expected to trust the product without being given the reasoning. Brief, evidence-informed tooltips (e.g., "Blink rate drops to 4-6 per minute during screen use. This reminder helps restore normal tear film distribution.") would reward curiosity, build trust, and differentiate the product from competitors that assume rather than explain.

### 6.4 Voice System
Both competitors use a calm-neutral premium tone with no textual personality. Neither offers a playful or internet-native mode. The category has room for a product that feels like a person, not a policy.

### 6.5 Display-Off Breaks
As noted in the subtle interaction catalog, a genuine black-screen or display-off break is a meaningful gap. It supports: deeper disengagement from the screen, energy and battery savings, stronger separation between work and rest, and a more honest interpretation of the 20-20-20 intent (look somewhere other than a screen) than a softened blurred overlay.

### 6.6 Return-to-Work Friction
Current competitors end breaks with a chime. There is no product-level attention given to *how the user returns* — what they see, how they feel, whether the transition back encourages a blink, a posture reset, or a moment of re-focus. The return from break is an underused design surface.

### 6.7 Meaningful Minimal Dashboard
Both competitors have stats dashboards. LookAway's is more complete. Neither is built with the philosophy "show only what changes behavior." A minimal dashboard that surfaces one number (recovery state), one streak (adherence), and one historical line (session shape today) — without overwhelming the user — would be more useful than a data dump that rewards looking at the app rather than away from the screen.

***

## 7. Design Philosophy Canvas for Fable 5

This section describes what the product should feel like, evoke, and communicate. It does not specify UI components, layout coordinates, or code structures. Fable 5 owns those decisions.

### 7.1 The Emotional Register

The app should feel like a calm, trusted presence — not an authority, not a cheerleader, not a nag. It lives in the background like a very good friend who happens to know a lot about ergonomics. It does not lecture. It does not panic. It does not demand. It suggests, nudges, and when necessary, holds firm — all with the same quiet confidence.

The product should feel warm without being soft, clear without being cold, and occasionally a little funny without ever being embarrassing.

### 7.2 Visual Atmosphere — Glass, Frost, Fog

The visual system should feel like light coming through frosted glass on a calm morning. Glassmorphism, used well, creates depth through translucency — foreground elements appear to float above their context, distinguished by soft light borders and diffused background blur rather than hard-edged containers. When applied to a wellness overlay or tray panel, this communicates something important: *this is not part of your work. It is hovering above it, offering you a moment.*

The frost and fog aesthetic should be used with restraint — not as decoration applied everywhere, but as the defining quality of break surfaces, overlay windows, and reminder states. The rest of the interface should be clean, minimal, and nearly invisible. The glass effect should feel earned, not generic.

Color should be nearly absent except as state signaling. The surface language should live in whites, near-whites, very pale warm grays, and translucency — with a single controlled accent that communicates health state without competing for visual attention. Think early morning, not neon medical dashboard.

### 7.3 Motion Language

Motion should be gentle, inevitable, and calm. Nothing should snap. Nothing should bounce aggressively. The app's ambient presence should feel like breathing — surfaces that soften in and soften out, not pop and slide. The language of transition should communicate that a break is natural, not disruptive.

When a break is genuinely urgent — when enforcement escalates — the motion should *not* become aggressive. It should become quieter. More certain. More present. Like a hand resting on a shoulder rather than an alarm sounding.

### 7.4 Density and Presence

The product should occupy as little visual and cognitive real estate as possible. The ideal state is near-invisibility: a quiet icon, a slow countdown at the cursor periphery, and nothing else. Every element that appears should have a reason to be there and a clear exit. The settings panel should feel spacious and considered, not overwhelming. The stats view should be digestible at a glance.

The rule of density: show less than you know. The app can hold rich session data without surfacing all of it at once.

### 7.5 Color as Health State

Rather than using color decoratively, color should function as a health state signal. The system should have a quiet language: a certain warmth when recovery state is good, a gentle shift when strain is accumulating, a clear signal when the user is overdue. This does not require bright traffic lights or dramatic red states. The shift should be subtle enough to be sensed before it is consciously noticed — like ambient lighting changing in a room.

Green, yellow, and red as concepts can be retained, but they should be expressed as soft tonalities rather than hard-coded UI badges.

### 7.6 The Break Surface

The break overlay or break window is the most important design surface in the product. It should feel like a deliberate pause — not a system error, not an intrusion, not a childish animation. The message displayed should feel personally relevant, human, and calm. The surface should be generous with space. The eye should have nowhere to rush.

The break message should be the only thing that matters during a break. Supporting elements — time remaining, skip/delay controls if allowed — should be present but peripheral, subordinate to the moment of stillness.

### 7.7 The Playful Mode

When the user enables playful mode, the character of the copy shifts — not the visual system. The glass and frost remain. The motion remains. Only the words become a little more alive, a little more internet-aware, a little more like something a peer would say. "Touch grass twin" should feel natural when it appears, not like a brand trying hard. The switch between calm and playful modes should feel like a persona setting, not a different product.

A useful mental model: the premium default is the app speaking in its natural voice. The playful mode is the same app after knowing the user for a while.

### 7.8 The Settings Experience

Settings should feel like a conversation, not a form. Every option that affects the user's health or experience should have a brief, honest explanation available on hover or tap — not as mandatory reading, but as a reward for curiosity. The rationale should be serious, concise, and evidence-informed: one sentence, one source-quality reference, no alarmism.

The onboarding should be short. Ideally, the user makes two or three choices — strictness mode, tone mode, office hours — and the app is configured well enough to start. Advanced settings exist for those who want them, but the defaults are good for everyone else.

***

## 8. Voice & Writing System

### 8.1 Default Voice

Calm. Human. Premium. Warm without being cloying. The writing should sound like a thoughtful person who respects the user's intelligence and takes their health seriously without being preachy. Each message should feel like it was written for an individual, not broadcast to users.

**Characteristics:**
- Short sentences. Rarely more than one clause.
- Present tense. Alive, not administrative.
- Second person when addressing the user directly; impersonal when describing the world.
- Never lecturing, explaining, or justifying.
- Never using medical language that implies disease.
- Occasionally poetic but never gratuitous.

**Tone examples (verified LookAway-inspired reference):**
- "Eyes to the horizon."
- "You've earned this."
- "Give your eyes a holiday."
- "Blink and breathe."
- "Stretch your shoulders."

**Original tone examples for this product (illustrative, not final):**
- "Look somewhere far away. Your eyes will thank you."
- "A twenty-second reset starts now."
- "Step back. Your screen can hold this thought."
- "Good time to blink slowly. Twice."
- "Your next session starts fresh."

### 8.2 Playful Optional Mode

When enabled, this mode shifts copy toward internet-native, warm-witty, lightly Gen Z phrasing. The key constraint: it should never feel like a brand trying to be cool. It should feel like the app itself has a personality. The wit should be dry and light, not exclamatory.

**Illustrative examples (not final):**
- "Touch grass, twin. Twenty seconds, go."
- "Your eyes are not it rn."
- "Bestie, look out the window for a sec."
- "You've been staring for a while. We see you."
- "Blink. No, actually blink. Full blink."
- "Long session detected. You're cooked. Rest."

### 8.3 Message Families

The copy system should be organized by purpose, not just tone. These families should be present:

| Family | Purpose | When it appears |
|---|---|---|
| Far-focus | Encourage 20-foot distance viewing, window look, horizon | Short break overlay |
| Blink | Prompt full, slow, complete blinks | Blink reminder |
| Posture | Gentle cue to unclench jaw, drop shoulders, sit tall | Posture reminder |
| Rest reward | Affirm the user for taking a break, frame rest positively | Break start, long break |
| Pre-break heads-up | Alert without pressure | 60 seconds before break |
| Return to work | Soft re-entry, posture reset, focus reset | Break end |
| Strain warning | Escalating awareness of accumulated session time | Skip debt warning state |

### 8.4 Evidence Tooltip Copy

Each setting in the app that relates to health behavior should have an optional hover tooltip with concise, serious, evidence-informed rationale. These should not be preachy. They should be informational and brief.

**Examples:**
- On blink reminder: "Blink rate drops from 15+ per minute to 4-6 during active screen use. Full blinks are needed to distribute tears across the cornea."
- On break interval: "Frequent short breaks reduce eye irritation and accommodation fatigue. Self-paced or frequent breaks may outperform rigid 20-minute intervals."
- On posture reminder: "Eye strain and neck-shoulder pain co-occur in over 63% of screen-heavy users. Posture correction during screen sessions reduces both."

***

## 9. Smart Break & Recovery-State Model

### 9.1 The Problem with Flat-Enforcement Apps

Both LookAway and DeskRest offer enforcement *levels* (strict, balanced, lenient) as static settings — configured once, applied uniformly every break. This is a reasonable starting point but misses the dynamic reality of screen fatigue: the fifth consecutive skipped break in a two-hour session is physiologically and behaviorally different from the first skipped break in a light 30-minute session.

### 9.2 A Three-State Recovery Model

This product should operate with a dynamic session-state model that changes the character of its communication based on accumulated session behavior:

**State: Clear**
The user has taken recent breaks, session is within healthy range, no skip debt. The app is nearly silent. Cursor timer shows quietly. Pre-break reminder appears gently and dissolves.

**State: Accumulating**
One or two breaks skipped, or session running longer than intended. The pre-break reminder lingers slightly longer. The break message shifts to acknowledge the extension: "Longer than usual. Your eyes are working hard." Color state shifts subtly warmer. The skip button still works.

**State: Due**
Three or more breaks skipped, or session is significantly overextended. The break message is more present, more direct. The app does not hide this state. In lenient mode, a color warning appears and the message is firm but not forced. In smart break mode, this can trigger an unskippable break if the user configured it.

### 9.3 Modes

| Mode | Character | Forced breaks | Skip behavior |
|---|---|---|---|
| **Lenient** | Soft color warnings, no enforcement | Never | Always available |
| **Smart Break (default)** | Escalating state, firm nudges | After repeated ignores, if enabled | Available with brief delay |
| **Focused / Hardcore** | No skipping, full enforcement | Always | Disabled |

These modes should be prominently offered during onboarding. Smart Break is the recommended default.

### 9.4 Recovery Score vs. Screen Score

LookAway's Screen Score is a daily summary. This product should have both a daily summary *and* a live session recovery state — a number or visual indicator that reflects the present moment, not just the accumulated day. The live state should be ambient and unobtrusive. The daily summary is for reflection, not real-time guidance.

### 9.5 Long-Session Behavior

Extended sessions — over 90-120 minutes without meaningful recovery — should trigger a different response than a normal break cycle. The app should recognize long sessions as qualitatively different from routine break debt and suggest or enforce a longer recovery break (5+ minutes) rather than another 20-second micro-break. This is supported by the evidence that 20-second breaks may be insufficient to allow full dissipation of NITM and accommodative fatigue.

***

## 10. Windows / Tauri Feasibility Notes

### 10.1 Why Tauri Is a Good Fit

A tray-first wellness utility needs to: start fast, idle with minimal RAM, spawn small windows on demand, and stay out of the user's way. This profile is poorly served by Electron (which bundles a full Chromium runtime) and well served by Tauri, which uses the system WebView2 on Windows. The result is smaller binaries, faster startup, and substantially lower idle memory. Tauri supports all the primitives this product needs: system tray creation, tray menus, tray event handling, multi-window management, transparent frameless windows, always-on-top windows, and global shortcut registration.

A tray-exclusive app is well-supported in Tauri: the default window can be removed from `tauri.conf.json`, leaving only the tray icon, and windows can be spawned and hidden on demand.

### 10.2 Overlay and Break Windows

Tauri supports transparent, frameless, always-on-top windows on Windows. A full-screen break overlay can be implemented as a transparent window set always-on-top, with the frosted-glass aesthetic applied via CSS backdrop-filter or WebView2's native fluent overlay mode (requires WebView2 runtime 125.0.2535.41 or higher). A developer community test in 2026 confirmed that transparent always-on-top overlays work reliably in Tauri v2 on Windows and macOS for production builds — the feared transparency-in-production bug did not reproduce.

For cursor-adjacent reminders, Tauri's window positioning APIs allow placing a small window near the cursor position. Selective click-through (passing mouse events through transparent areas while keeping clickable UI panels interactive) requires a native Rust workaround using `SetWindowLongPtrW` on Windows to toggle `WS_EX_TRANSPARENT` based on cursor position.

### 10.3 Global Shortcuts

Tauri v2 provides a `tauri-plugin-global-shortcut` plugin that registers system-wide keyboard shortcuts. These fire even when the app is not in focus, which is the correct behavior for tray-resident apps. On Windows, global shortcuts do not require accessibility permissions (unlike macOS, where they can trigger a permission prompt). The implementation is straightforward via both JavaScript and Rust APIs. Conflicts with other registered shortcuts should be handled gracefully — the product should work even without the shortcut if registration fails.

### 10.4 System Tray Specifics

Tauri v2's system tray documentation covers: creating a tray icon, building tray menus, responding to tray events (left click, right click, double click), dynamically updating the tray icon (e.g., to reflect health state color), and hiding the app from the taskbar while keeping it in the tray. Dynamic tray icon updates can be used to reflect recovery state — a subtle color or glyph change that communicates strain level without requiring the user to open the app.

### 10.5 Permissions & Accessibility

For the core feature set — tray presence, break overlays, timers, notifications, keyboard shortcuts, idle detection — **no elevated permissions or UIAccess are required on Windows**. Standard desktop app permissions are sufficient. UIAccess is a Windows feature for assistive technologies that allows interaction with protected system UI (like UAC dialogs and the task manager); it requires a signed binary installed in Program Files and carries administrative trust implications. This product does not need UIAccess. Idle time detection can be accomplished via standard Windows API polling without elevation.

### 10.6 Code Signing and SmartScreen

Windows Defender SmartScreen evaluates downloaded executables against a reputation database. Unsigned or unrecognized apps trigger a warning ("Windows protected your PC"). The signing and reputation situation in 2026 is as follows:

- **EV certificates no longer grant immediate SmartScreen bypass.** Since 2024, both OV and EV certificates go through the same reputation-building process. EV still removes the "Unknown Publisher" UAC dialog line and may build reputation somewhat faster than OV.
- **OV (Organization Validation) RSA certificates** display the organization's verified name in security dialogs and are the practical starting choice for a new app.
- **RSA is the recommended algorithm** for SmartScreen compatibility. Microsoft's documentation cautions against ECC/ECDSA for SmartScreen and Smart App Control.
- **Reputation builds over time** through download volume and clean analysis. A new certificate starts with no reputation; the first wave of downloads will still show a warning until sufficient trust accumulates.
- **Signing every binary and installer consistently** and using a timestamp server (so signatures remain valid after certificate expiry) are the minimum hygiene requirements.
- **Installing to Program Files** improves trust posture for SmartScreen and is also required if UIAccess is ever added later.
- Tauri's distribution documentation explicitly covers Windows code signing, including GitHub Actions integration.

The practical advice: get an OV RSA certificate, sign every build, timestamp every signature, ship to Program Files, and accept that the first several hundred downloads will show a SmartScreen warning. Social proof on the download page (user count, review quotes) helps users past the warning.

***

## 11. Recommended Product Scope

### 11.1 Must-Have at Launch

These features constitute the minimum viable product that can stand in the Windows eye-care utility category:

- Tray-resident background presence with live status icon
- Short breaks with configurable interval and duration
- Pre-break heads-up notification with configurable timing
- Skip / delay / snooze logic
- Full-screen break overlay with message display
- Typing-aware break delay (hold break until user stops typing)
- Idle time detection with pause and reset thresholds
- Smart pause: meetings/video/fullscreen app detection
- Blink reminders (configurable interval, separate from breaks)
- Posture reminders (subtle, non-blocking)
- Office hours / working hours schedule
- Three strictness modes (Lenient / Smart Break / Focused)
- Calm default copy + optional playful mode (off by default)
- Sensible default profile that works without configuration
- OV-signed installer, installed to Program Files

### 11.2 Strong Differentiators (Phase 2)

These features do not exist in the category at this quality level and represent meaningful product differentiation:

- **Dynamic break debt / escalation model** — recovery state changes the character and intensity of reminders within a session
- **Live recovery state indicator** — ambient always-visible session health signal
- **Display-off / black-screen break mode** — optional, togglable
- **Evidence microcopy** — per-setting hover tooltips with brief evidence-informed rationale
- **Two-layer copy system** — calm default + playful optional mode as a proper toggle
- **Return-from-break surface** — intentionally designed end-of-break moment
- **Long-session detection with long-break escalation** — recognizes extended sessions and upgrades break type

### 11.3 Optional / Later Features

These are valuable but not essential for v1 and should not delay launch:

- Custom exercise routines per break type
- Cursor-adjacent timer (complex to implement cleanly, but high user value)
- Clock Out / end-of-workday boundary feature
- Per-app deep focus configuration UI
- Full stats dashboard with session history and daily score
- Streak tracking and activity calendar
- Automation hooks (run script / shortcut on break start/end)
- Theme customization for break overlays

***

## 12. Final Creative Brief for Fable 5

Fable 5, you are the visual and interaction lead on this product. What follows is the strategic and emotional intent. The composition, component choices, motion specifics, and visual execution are yours.

***

**What this product is:**
A Windows tray application that helps desk workers rest their eyes, reset their posture, and recover from long screen sessions. It lives quietly in the background, knows when to stay out of the way, and surfaces reminders at the right moments. It is built for developers, designers, students, and knowledge workers who sit at their screens for hours and sometimes forget they have a body.

**What it should feel like:**
Premium, calm, and alive. Like morning light through frosted glass. Like a product made by someone who uses it every day. Minimal to the point of near-invisibility in its resting state, and gracefully present when it matters. Not corporate. Not clinical. Not gamified. Not cheap.

**The visual grammar:**
Glass. Frost. Fog. Translucency. Soft light. Near-white surfaces with depth. A single accent that communicates health state. Near-absent color in resting states. Gentle, inevitable motion — nothing snaps, nothing bounces. Surfaces that feel like they exist slightly above the desktop, hovering, not embedded.

**The character:**
Calm, thoughtful, occasionally a little witty when the user has asked for that. It should feel like the product has a personality — not a brand personality, but a person's personality. Warm without being cloying. Clear without being cold. It knows things but doesn't lecture. It cares but doesn't panic.

**The break surface:**
The most important design surface. During a break, nothing should compete with the moment of rest. The message should be the dominant element. Supporting controls should be present but subordinate. The surface should communicate: *this moment is yours. Look somewhere else. Breathe. Come back fresh.*

**The tray presence:**
Nearly invisible. A small icon that subtly communicates health state through color or form. Quick access to pause, take a break now, or open settings. Nothing aggressive. Nothing that makes the user feel watched.

**The settings experience:**
Spacious. Explanatory. Every option should feel considered, not bureaucratic. Evidence tooltips are available on hover — short, serious, trustworthy. The onboarding is short: choose your strictness mode, choose your tone, set your hours. Done.

**The copy system:**
Two layers. Default is warm, clear, short. Optional playful mode is internet-native, lightly witty, still tasteful. Both use short sentences, present tense, and second-person address. Neither lectures. The playful mode is a persona shift on the same product, not a different product.

**The recovery-state model:**
The app knows how the session is going. Clear means quiet. Accumulating means warmer and more present. Due means firm and more visible. These states change the character of reminders without changing the product's calm underlying temperament. This is expressed visually in the product's color state, the weight of the break message, and the behavior of the reminder surface.

**What to avoid:**
- Neon colors or health-app dashboards with red alert states
- Gamification that feels cheap (badge counts, level-up language, confetti for taking a break)
- Clutter in the tray panel or settings
- Copy that sounds like it came from a wellness brand's marketing team
- Any UI language that implies the app knows the user's health or diagnoses anything
- Heavy, aggressive, or bouncy animation
- Generic frosted glass that looks like every other macOS app from 2022

**What to preserve:**
The intelligence of the product. It knows context. It holds back. It escalates carefully. It speaks clearly. It earns trust by not overusing its own presence. The rarest interaction in this product is the forced break — and when it arrives, it should feel inevitable, not violent.

This is a product that says: *take care of yourself* — and then quietly waits while you do.

***

*Research compiled June 2026. Scientific references: PubMed, NIH PMC, NIOSH, Review of Optometry. Competitor data: lookaway.com/docs, deskrest.com/docs, and their respective public materials. Windows platform references: Microsoft Learn, Tauri v2 documentation, community implementation reports.*