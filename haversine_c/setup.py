# -*- coding: utf-8 -*-
from setuptools import setup
from build import *

setup_kwargs = {
    'name': 'haversine',
    'version': '1.0',
    'packages': ['haversine'],
    'package_data': {'': ['*']},
    'python_requires': '>=3.6,<4.0',
}

build(setup_kwargs)
setup(**setup_kwargs)
