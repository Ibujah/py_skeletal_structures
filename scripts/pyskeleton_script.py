import os

from py_skeletal_structures.skeleton2d import PySkeleton2D, save_pyskeleton2d_ply

import matplotlib.pyplot as plt

if __name__ == "__main__":
    skel = PySkeleton2D()

    i1 = skel.insert_vertex([0., 0.], 1.0)
    i2 = skel.insert_vertex([1., 0.], 0.25)
    i3 = skel.insert_vertex([0., 1.], 0.25)

    skel.insert_edge(i1, i2)
    skel.insert_edge(i1, i3)

    if not os.path.exists("output"):
        os.mkdir("output")
    save_pyskeleton2d_ply('./output/skel.ply', skel, "c'est un test d'écriture")

    plt.figure()
    for i1 in range(skel.get_nb_vertices()):
        v1 = skel.get_vertex_coords(i1)
        plt.plot(v1[0], v1[1], 'r+')
        for i2 in skel.get_neighbors(i1):
            v2 = skel.get_vertex_coords(i2)
            plt.plot([v1[0], v2[0]], [v1[1], v2[1]], 'r')
    plt.axis('equal')
    plt.show()

