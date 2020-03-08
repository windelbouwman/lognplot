""" Helper script to assemble a folder which can be zipped and shared on windows.
"""


import contextlib
import argparse
import os
import subprocess
import shutil


# required DLL files by the executable:

required_dlls = [
    "atk-1",
    "bz2",
    "cairo",
    "cairo-gobject",
    "epoxy-0",
    "expat",
    "fontconfig",
    "freetype",
    "gdk_pixbuf-2",
    "gdk-3",
    "gio-2",
    "glib-2",
    "gmodule-2",
    "gobject-2",
    "gtk-3",
    "harfbuzz",
    "hdf5",
    "libcharset",
    "libiconv",
    "libintl",
    "libpng16",
    "pango-1",
    "pangocairo-1",
    "pangoft2-1",
    "pangowin32-1",
    "pcre",
    "zlib1",
]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "vcpkg_install_folder",
        help=r"Specify the vcpkg install folder. For example: C:\vcpkg\installed\x64-windows",
    )
    args = parser.parse_args()

    # Calculate various folder paths:
    this_folder = os.path.dirname(os.path.abspath(__file__))
    root_folder = os.path.normpath(os.path.join(this_folder, ".."))
    dist_folder = os.path.join(root_folder, "gtkdist")
    vcpkg_folder = args.vcpkg_install_folder

    build(this_folder, vcpkg_folder)
    gather_distribution(dist_folder, root_folder, vcpkg_folder)


def build(lognplotgtk_folder, vcpkg_install_folder):
    new_env = os.environ.copy()
    new_env["GTK_LIB_DIR"] = os.path.join(vcpkg_install_folder, "lib")
    new_env["LIB"] = os.path.join(vcpkg_install_folder, "lib")
    new_env["PATH"] = os.path.join(vcpkg_install_folder, "bin") + ";" + new_env["PATH"]
    # Allow hdf5 crate to build properly:
    new_env["HDF5_DIR"] = vcpkg_install_folder
    
    # set VCPKGDIR=c:\vcpkg
    # set GTK_LIB_DIR=%VCPKGDIR%\installed\x64-windows\lib
    # set LIB=%GTK_LIB_DIR%
    # set PATH=%VCPKGDIR%\installed\x64-windows\bin;%PATH%

    cmd = ["cargo", "build", "--release"]
    with pushd(lognplotgtk_folder):
        print('Running command: {}'.format(' '.join(cmd)))
        subprocess.run(cmd, check=True, env=new_env)


def gather_distribution(dist_folder, root_folder, vcpkg_folder):
    # Make release folder:
    if os.path.exists(dist_folder):
        print(f"Dist folder ({dist_folder}) already present")
    else:
        print(f"Creating dist folder {dist_folder}")
        os.makedirs(dist_folder)

    # Copy release build:
    dist_lognplotgtk_exe = os.path.join(dist_folder, "lognplotgtk.exe")
    release_lognplotgtk_exe = os.path.join(
        root_folder, "target", "release", "lognplotgtk.exe"
    )
    copy_if_missing(dist_lognplotgtk_exe, release_lognplotgtk_exe)

    # Copy various dll's required
    for required_dll in required_dlls:
        dll_filename = required_dll + ".dll"
        dist_dll = os.path.join(dist_folder, dll_filename)
        vcpkg_dll = os.path.join(vcpkg_folder, "bin", dll_filename)
        copy_if_missing(dist_dll, vcpkg_dll)


@contextlib.contextmanager
def pushd(folder):
    old_dir = os.getcwd()
    print(f"Entering {folder}")
    os.chdir(folder)
    yield
    print(f"Restoring {old_dir}")
    os.chdir(old_dir)


def copy_if_missing(dist_file, original_file):
    if os.path.exists(dist_file):
        print(f"{dist_file} already present")
    else:
        print(f"Copying {original_file} to {dist_file}")
        shutil.copyfile(original_file, dist_file)


if __name__ == "__main__":
    main()
