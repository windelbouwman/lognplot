

# Linux build instructions

Install the proper development packages for gtk and such.

Next, build the GTK application like this:

    $ cd lognplotgtk
    $ cargo build --release

# MacOS build instructions

TODO

# Windows build instructions

For windows, use [vcpkg](https://github.com/Microsoft/vcpkg) to install GTK development packages:

    CMD> vcpkg install gtk:x64-windows

Next up, check the vcpkg library folder `vcpkg\installed\x64-windows\lib`
and make sure you have
`gtk-3.0.lib` by copying/symlinking/replicating/renaming `gtk-3.lib`.
The same might hold for `gdk-3.lib`.

Next, setup some environment variables to allow the gtk-rs crate to pickup gtk:

    CMD> set VCPKGDIR=c:\git\vcpkg-folder
    CMD> set GTK_LIB_DIR=%VCPKGDIR%\installed\x64-windows\lib
    CMD> set LIB=%GTK_LIB_DIR%
    CMD> set PATH=%VCPKGDIR%\installed\x64-windows\bin;%PATH%

Then, build the application:

    CMD> cargo build --release

At this point, you have a runnable application, but icons and themes
are still defunct.

To fix this, download two icon packages from msys2:

- hicolor-icon-theme
- adwaita-icon-theme

Extract the packages and copy the contents of the `share/icons` folder into the
folder `%VCPKGDIR%\installed\x64-windows\share\icons`. You should now have
the folder structure:

- `x64-windows/share/icons`
    - `Adwaita`
        - `index.theme`
        - `16x16` and other folders with icons of a certain size.
    - `hicolor`
        - `index.theme`
        - `16x16` and other folders with icons of a certain size.

Now the app will contain the proper icons. To theme this further you could
install a windows 10 or windows 7 theme.

## Distribution build script

To assist in all this stuff, a python script is available which does some of
the steps:

    CMD> python lognplotgtk\make_win32_distro.py my_vcpkg_install_folder

References:
- https://www.gtk.org/docs/installations/windows/
- https://www.reddit.com/r/rust/comments/bzkhmt/how_to_use_gtkrs_on_windows_using_the_msvc/
