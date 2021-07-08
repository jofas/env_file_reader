CHANGELOG
=========

The format is based on `Keep a Changelog <https://keepachangelog.com/en/1.0.0/>`_,
and this project adheres to `Semantic Versioning <https://semver.org/spec/v2.0.0.html>`_.


[0.2.0]
-------

Changed
^^^^^^^

* Path parameters for ``read_file`` and ``read_files`` now generic
  over ``AsRef<Path>`` instead of ``&str``

* Keys are now case sensitive (before they were lowercased to spare
  the eyes)

* Keys can now also contain the hyphen as character


[0.1.0]
-------

Added
^^^^^

* ``read_file`` function

* ``read_files`` function
