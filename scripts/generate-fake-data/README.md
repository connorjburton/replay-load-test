To run the script, first you need to build the image

`docker build . -t generate-fake-data`

Then you can run this script with a volume mount to retrieve the output data in the `./data` folder.

`docker run -v %cd%:/gen gen-fake`

If on MacOS you can swap out `%cd%` for `$PWD`

Note that as this is a volume, the whole `generate-fake-data` folder will be mounted in the docker image, so consecutive runs will be mounting previously generated data into the image. Therefore the generated JSON file has a timestamp to avoid conflicts.
