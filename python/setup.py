from setuptools import setup, find_packages

setup(
    name="lognplot",
    version="0.1.0",
    author="Windel Bouwman",
    description="Log and plot data. This project basically implements a software scope.",
    url="https://github.com/windelbouwman/quartz",
    packages=find_packages(),
    license="GPLv3",
    classifiers=[
        "Topic :: Scientific/Engineering :: Visualization",
        "Topic :: System :: Monitoring",
        "Topic :: System :: Logging",
        "License :: OSI Approved :: GNU General Public License v3 (GPLv3)",
    ],
)
