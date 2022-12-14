from setuptools import setup, find_packages

setup(
    name="mindflow",
    version="0.1.5",
    py_modules=["mindflow"],
    entry_points={"console_scripts": ["mf = mindflow.main:main"]},
    packages=find_packages(),
    install_requires=["requests", "bs4", "chardet", "pyperclip", "gitpython"],
)
