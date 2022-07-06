Vagrant.configure("2") do |config|
  config.vm.box = "gusztavvargadr/windows-10"
  config.vm.network :forwarded_port, guest: 8009, host: 8000,
  config.vm.network :forwarded_port, guest: 8008, host: 8001,
  config.vm.network :forwarded_port, guest: 30007, host: 30000,
  config.vm.network :forwarded_port, guest: 4447, host: 4444,
  config.vm.synced_folder  ".", "C:\\Windows\\system32\\vagrant", disabled: false
  config.vm.provision "shell", path: "./run_test.ps1"
end