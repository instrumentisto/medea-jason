
Set-Location ./vagrant/test;
$env:WEBRTC_FAKE_MEDIA = 'true';
$env:IP_TEST_BASE = '10.0.2.2';
.\medea_jason_example.exe -wait | ForEach-Object { Write-Output $_; 
    if ($_ -match "All tests passed!") {
        exit 0
    }
    elseif ($_ -match "Some tests failed.") {
        exit 1
    } } ;