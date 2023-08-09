#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import shutil
import tempfile
from pathlib import Path

import pdoc

if __name__ == '__main__':
    tmpdir = tempfile.TemporaryDirectory()
    package_name = 'solrstice'
    package_path = os.path.join(tmpdir.name, package_name)
    current_directory = os.path.dirname(os.path.realpath(__file__))
    docs_directory = Path(os.path.join(Path(current_directory), 'docs'))

    os.mkdir(package_path)
    for filename in os.listdir(package_name):
        f = os.path.join(package_name, filename)
        if os.path.isfile(f) and filename.endswith('.pyi'):
            shutil.copyfile(f, os.path.join(package_path, filename[:-1]))
    shutil.copyfile('README.md', os.path.join(tmpdir.name, 'README.md'))
    with open(os.path.join(package_path, '__init__.py'), 'w') as f:
        f.write('''
"""
.. include:: ../README.md
"""
    ''')

    pdoc.pdoc(os.path.join(tmpdir.name, package_name), output_directory=docs_directory)
