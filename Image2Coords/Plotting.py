from sklearn.cluster import DBSCAN
from matplotlib import pyplot as plt
import numpy as np
import argparse
from ImageToCSV import PixelLocationInfo, tolerance_check, tol_comp, TOLERANCE, ImageInfo

def plot_scatter(ax, X, Y):
    ax.scatter(X, Y)
    ax.set_title('Scatter Plot of Motion Positions')
    ax.set_xlabel('X Position')
    ax.set_ylabel('Y Position')

# https://www.geeksforgeeks.org/dbscan-clustering-in-ml-density-based-clustering/
def plot_clusters(ax, coordinates):
    coords = [(coord.X, coord.Y) for coord in coordinates]
    X = np.array(coords)
    clustering = DBSCAN(eps=3, min_samples=2).fit(X)
    labels = clustering.labels_

    unique_labels = set(labels)
    colors = [plt.cm.Spectral(each) for each in np.linspace(0, 1, len(unique_labels))]
    
    for k, col in zip(unique_labels, colors):
        if k == -1:
            # Black is noise
            col = [0, 0, 0, 1]

        class_member_mask = (labels == k)

        xy = X[class_member_mask]
        ax.plot(xy[:, 0], xy[:, 1], 'o', markerfacecolor=tuple(col),
                markeredgecolor='k', markersize=6)

    ax.set_title('DBSCAN Clustering')
    ax.set_xlabel('X Position')
    ax.set_ylabel('Y Position')

def process_image(image_name):
    image1 = ImageInfo(image_name)
    image1.initalize_pixel_info()
    image1.get_motion_position()

    X = [coord.X for coord in image1.Pixel_Coords]
    Y = [coord.Y for coord in image1.Pixel_Coords]

    #  two subplots
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(image1.width/ 100, image1.height/ 100))
    plot_scatter(ax1, X, Y)

    plot_clusters(ax2, image1.Pixel_Coords)

    plt.tight_layout()
    plt.show()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('image_name', type=str, help='The name of the image file to process')
    args = parser.parse_args()

    process_image(args.image_name)

if __name__ == "__main__":
    main()
