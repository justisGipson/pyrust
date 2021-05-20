from setuptools import setup
from setuptools_rust import RustExtension, Binding

setup(
  name="pyrust",
  version="0.0.1",
  rust_extension=[RustExtension("pyrust.pyrust", binding=Binding.PyO3)],
  packages=["pyrust"],
  zip_safe=False,
)
