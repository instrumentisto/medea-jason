
Set-Location ./vagrant;
Expand-Archive ./Release.zip -DestinationPath ./;
Set-Location ./Release;
$env:WEBRTC_FAKE_MEDIA = 'true';
.\medea_jason_example.exe -wait | ForEach-Object { Write-Output $_; 
    if ($_ -match "All tests passed!") {
        exit 0
    }
    elseif ($_ -match "Some tests failed.") {
        exit 1
    } } ;