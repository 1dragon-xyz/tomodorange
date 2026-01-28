# Michael's Pomodoro

**A minimalist, "calm technology" Pomodoro timer designed for Deep Focus on Windows 11.**

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Python](https://img.shields.io/badge/python-3.10%2B-blue)

## Why This App? (The Philosophy)

Most Pomodoro apps are stressful. They flash, tick, and demand your attention. **Michael's Pomodoro** is built to be ignored.

*   **🚫 No Ticking Anxiety**: The "Minutes Only" display (e.g., "25") removes the urgency of watching seconds count down.
*   **👻 Ghost Mode**: A unique feature that makes the widget **click-through**. It floats over your work like a watermark, impossible to accidentally drag or close.
*   **🌊 Auditory Flow**: Uses a rhythmic **Water Drop** sound (every 1 second) during Work to induce flow, and ambient **Beach Waves** during Break to encourage relaxation.
*   **🔒 Strict Workflow**: There is **No Pause Button**. This is intentional. You either commit to the block, or you stop.
*   **🎨 Seamless Design**: Frameless, transparent, and customizable to blend perfectly with your desktop wallpaper.

## Features

*   **Always-on-Top Floating Widget**: Stays visible but unobtrusive.
*   **Smart Transitions**: Automatically switches between Work (default 25m) and Break (default 5m) phases.
*   **System Tray Hub**: All controls are tucked away in the right-click tray menu to keep your screen clean.
*   **Customizable Aesthetics**: Adjust Opacity, Text Size, Color, and Audio Volumes to your exact preference.
*   **Lightweight**: Designed to use minimal system resources (<1% CPU).

## Installation

### Prerequisites
*   Windows 10/11
*   Python 3.10 or higher

### Setup
1.  **Clone the repository**:
    ```bash
    git clone https://github.com/1dragon-xyz/michaels-pomodoro.git
    cd michaels-pomodoro
    ```

2.  **Install dependencies**:
    ```bash
    pip install -r requirements.txt
    ```

3.  **Run the app**:
    ```bash
    python src/main.py
    ```

## Usage Guide

### The Floating Widget
*   **Move**: Click and drag the text to position it anywhere on your screen.
*   **Status**:
    *   **Teal Text**: Work Focus. 🧠
    *   **Green Text**: Break Time. ☕

### The System Tray (Right-Click Menu)
The "Tomato" icon in your system tray is your control center.
*   **Settings**: Open the configuration window to adjust transparency, size, and times.
*   **Ghost Mode**: Toggle this ON to make the widget click-through. (You must toggle it OFF from the tray to move the widget again).
*   **Mute**: Quickly silence all audio cues.

## Customization
Open **Settings** from the tray to:
*   Change Work/Break durations (changes apply to the *next* cycle).
*   Fine-tune the "Water Drop" and "Waves" volume independently.
*   Set the perfect transparency level for your specific wallpaper.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)
