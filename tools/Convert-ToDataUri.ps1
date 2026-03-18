$imagePath = "background.jpg"
$bytes = [System.IO.File]::ReadAllBytes($imagePath)
$base64 = [System.Convert]::ToBase64String($bytes)
$ext = [System.IO.Path]::GetExtension($imagePath).TrimStart('.').ToLower()
$dataUri = "data:image/$ext;base64,$base64"
 
# $dataUri | Set-Content "background.datauri.txt"
$dataUri | Set-Clipboard