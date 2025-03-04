from PIL import Image
import numpy as nb
import argparse
import numpy as np
from matplotlib import pyplot as plt
import os

#Header: Run as python3 ImageToCSV.py *.mp4
#Dependencies: numpy,PIL,argparse,matplotlib,os,conversion.sh
#Will utilize conversion.sh to convert a Video file in a an array of custom image objects that track the coordinates of pixels within
#a specified tolerance TOLERANCE of a desired color. Be warned is very slow at higher tolerances!!!!


TOLERANCE = 20
#Checks each RGB value to see if it's within specified tolerance
def tolerance_check(tup,r_val,g_val,b_val,r_tol=25,g_tol=25,b_tol=25) -> bool:
    if(tol_comp(tup,0,r_val,r_tol) and tol_comp(tup,1,g_val,g_tol) and tol_comp(tup,2,b_val,b_tol)):
        return True
    return False

#Checks a single RGB value to see if it's within specified tolerance, called by tolerance_check.
def tol_comp(tup,tup_index,val,tol):
    if(val - tol  <= tup[tup_index] and val + tol >= tup[tup_index]):
        return True
    return False

#Struct to track the x,y position of a pixel in image
class  PixelLocationInfo:
    def __init__(self,Xposition,Yposition):
        self.X = Xposition
        self.Y = Yposition
    def __str__(self):
        return f"({self.X},{self.Y})"

#Class to store ImageInfo objects
class VideoFileInfo:
    def __init__(self):
        self.ArrayOfImages = []

#Class to do all of the image processing. Used by declaring with image name, then running initalize_pixel_info followed by get_motion_position
class ImageInfo:
    def __init__(self,image_name):
        self.image_name = image_name
        self.Image_pixels = None
        self.width = 0
        self.height = 0
        self.Pixel_Coords = []
    def __str__(self):
        return f"{self.image_name}"
    def initalize_pixel_info(self):
        im = Image.open(self.image_name)
        pixels = im.load()
        self.width,self.height = im.size

        #Initalize NP array of RGB Tuples
        self.Image_pixels = np.empty((self.height,self.width),dtype =object)
        for x in range(self.height):
            for y in range(self.width):
                cpixel = pixels[y, x]
                self.Image_pixels[x][y] = cpixel
    def get_motion_position(self):
        X = []
        Y = []
        for x in range(self.height):
            for y in range(self.width):
                # print("Pixel:", self.Image_pixels[x][y])
                if tolerance_check(self.Image_pixels[x][y],0,0,0,TOLERANCE,TOLERANCE,TOLERANCE):
                    print("Pixel:", self.Image_pixels[x][y])
                    print("Location:", x, y)
                    X.append(x)
                    Y.append(y)
                    self.Pixel_Coords.append(PixelLocationInfo(x,y))
    def Print_PLI(self):
        for i in self.Pixel_Coords:
            print(i)


if __name__ == "__main__":
    #Gets command line argument for .mp4 to parse
    parser = argparse.ArgumentParser()
    parser.add_argument('file_name', type=str)
    args = parser.parse_args()

    #Runs Shell Script and stores the contents in a directory called Result(can modify to be provided by command line pretty easily just haven't done)
    # os.system("./conversion.sh " + args.file_name +  " Result")


    #Essentially Main of program. Parses the vide file, then prints out each image file that has been succesfully parsed.
    Vid_Info = VideoFileInfo()
    directory = "Result"
    for filename in os.listdir(directory):
        f = os.path.join(directory,filename)
        if os.path.isfile(f):
            x = ImageInfo(f)
            x.initalize_pixel_info()
            x.get_motion_position()
            Vid_Info.ArrayOfImages.append(x)
    for i in Vid_Info.ArrayOfImages:
        print(i)