Vagrant.configure("2") do |config|
  config.vm.box = "gusztavvargadr/windows-10"
  config.vm.network :forwarded_port, guest: 6379, host: 6379
  config.vm.network :forwarded_port, guest: 6565, host: 6565
  config.vm.network :forwarded_port, guest: 8000, host: 8000
  config.vm.network :forwarded_port, guest: 8001, host: 8001
  config.vm.network :forwarded_port, guest: 30000, host: 30000
  config.vm.network :forwarded_port, guest: 4444, host: 4444
  config.vm.synced_folder  ".", "C:\\Windows\\system32\\vagrant", disabled: false
  config.vm.provision "shell", path: "./run_test.ps1"
end