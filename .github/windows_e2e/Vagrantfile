Vagrant.configure("2") do |config|
  config.vm.box = "gusztavvargadr/windows-10"
  config.vm.synced_folder  ".", "C:\\Windows\\system32\\vagrant", disabled: false
  config.vm.provision "shell", path: "./run_test.ps1"
end