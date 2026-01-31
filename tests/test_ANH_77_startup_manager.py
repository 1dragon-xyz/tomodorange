
import os
import sys
import subprocess
import shutil

# Add src to sys.path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src'))

from utils.startup_manager import StartupManager

def test_startup_shortcut_creation():
    print("Testing startup shortcut creation...")
    
    # helper to read shortcut args using VBScript
    def get_shortcut_args(lnk_path):
        vbs_script = f"""
        Set oWS = WScript.CreateObject("WScript.Shell")
        Set oLink = oWS.CreateShortcut("{lnk_path}")
        WScript.Echo oLink.Arguments
        """
        vbs_path = "read_shortcut.vbs"
        with open(vbs_path, "w") as f:
            f.write(vbs_script)
        
        try:
            result = subprocess.run(["cscript", "//Nologo", vbs_path], capture_output=True, text=True, check=True)
            return result.stdout.strip()
        finally:
            if os.path.exists(vbs_path):
                os.remove(vbs_path)

    try:
        # Mock sys.argv[0] to simulate a path with spaces
        original_argv0 = sys.argv[0]
        # We need a fake path that 'exists' or acceptable for logic, but for shortcut creation
        # we mainly care about the text string in arguments.
        # But wait, startup_manager checks os.path.dirname(sys.argv[0]) to find assets.
        # If we change it to a non-existent path, asset check might fail/change behavior.
        # Let's just trust that we want to check if QUOTES are applied to whatever sys.argv[0] is.
        
        # Actually, let's force the method to use a path with spaces by modifying the class temporarily if possible, 
        # or just mocking sys.argv[0] and patching os.path.dirname/abspath if needed.
        # Simpler: Just set sys.argv[0] to a path with spaces. 
        # The code does: script_path = os.path.abspath(sys.argv[0])
        # It then does: src_dir = os.path.dirname(script_path)
        # It checks assets in src_dir/assets etc.
        # If we mock sys.argv[0], the assets won't be found, so it might default to some other icon.
        # This shouldn't crash the shortcut creation itself.
        
        fake_path_with_spaces = r"C:\Program Files\My App\src\main.py"
        sys.argv[0] = fake_path_with_spaces

        # Enable startup
        print(f"Enabling startup with mocked path: {sys.argv[0]}")
        result = StartupManager.set_run_at_startup(True)
        if not result:
            print("FAILED: set_run_at_startup returned False")
            return

        shortcut_path = StartupManager._get_shortcut_path()
        if not os.path.exists(shortcut_path):
            print(f"FAILED: Shortcut not found at {shortcut_path}")
            return

        print(f"Shortcut created at: {shortcut_path}")
        
        args = get_shortcut_args(shortcut_path)
        print(f"Shortcut Arguments: [{args}]")
        
        # Check if arguments are quoted
        # If the path has spaces, it MUST be quoted.
        if ' ' in args:
             if not (args.startswith('"') and args.endswith('"')):
                 print("FAIL: Arguments contain spaces but are not quoted.")
             else:
                 print("PASS: Arguments are correctly quoted.")
        else:
             print("FAIL: Test setup error - arguments should have spaces in this test case.")

    except Exception as e:
        print(f"ERROR: {e}")
    finally:
        # Restore sys.argv[0]
        if 'original_argv0' in locals():
            sys.argv[0] = original_argv0

        # Cleanup
        print("Cleaning up (removing shortcut)...")
        StartupManager.set_run_at_startup(False)

if __name__ == "__main__":
    test_startup_shortcut_creation()
