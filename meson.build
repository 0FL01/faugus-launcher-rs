project(
  'faugus-launcher',
  version: '1.13.5',
  meson_version: '>=1.2.0',
  license: 'MIT',
  license_files: 'LICENSE',
)

fs = import('fs')
gnome = import('gnome')

subdir('assets')
subdir('languages')
subdir('data')
if get_option('legacy_python')
  py = import('python').find_installation()
  subdir('faugus')
  install_data(
    'faugus_launcher.py',
    'faugus_run.py',
    'faugus_proton_manager.py',
    rename: [
      'faugus-launcher',
      'faugus-run',
      'faugus-proton-manager',
    ],
    install_mode: 'rwxr-xr-x',
    install_dir: get_option('bindir'),
  )
endif

install_data(
  'LICENSE',
  install_dir: get_option('datadir') / 'licenses' / meson.project_name(),
)

gnome.post_install(
  gtk_update_icon_cache: true,
)
