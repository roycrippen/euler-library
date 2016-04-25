Euler Library
=============

Rust library of common functions used to help solve Project Euler problems https://projecteuler.net/.

Please read the `API documentation here`__

__ http://roycrippen.github.io/euler_library/euler_library/index.html

|build_status|_ 

.. |build_status| image:: https://travis-ci.org/roycrippen/euler_library.svg?branch=master   
.. _build_status: https://travis-ci.org/roycrippen/euler_library

How to use with cargo::

    [dependencies]
    euler_library = { git = "https://github.com/roycrippen/euler_library" }

How to use in your crate:

.. code:: rust

    extern crate euler_library;

    use euler_library::common as eu;
