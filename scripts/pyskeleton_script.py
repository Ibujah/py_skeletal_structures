import os
import numpy as np

import py_skeletal_structures;
from py_skeletal_structures.skeleton2d import PySkeleton2D, load_pyskeleton2d_ply, save_pyskeleton2d_ply


if __name__ == "__main__":
    skel = PySkeleton2D()

    v1 = skel.insert_vertex([0., 0.], 1.0)
    v2 = skel.insert_vertex([1., 0.], 0.25)
    v3 = skel.insert_vertex([0., 1.], 0.25)

    skel.insert_edge(v1, v2)
    skel.insert_edge(v1, v3)

    if not os.path.exists("output"):
        os.mkdir("output")
    save_pyskeleton2d_ply('./output/skel.ply', skel, "c'est un test d'écriture")
