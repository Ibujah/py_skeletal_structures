import os
import numpy as np

from py_skeletal_structures.mesh3d import load_pymesh_obj, save_pymesh_ply


if __name__ == "__main__":
    mesh = load_pymesh_obj('./resources/cube.obj')

    print("nb vertices:", mesh.get_nb_vertices())
    for i in range(0, mesh.get_nb_vertices()):
        print(i, mesh.get_vertex(i))
    
    vert = np.array(mesh.get_all_vertices())
    print(vert)
    print(vert.shape, vert.dtype)

    print("nb faces:", mesh.get_nb_faces())
    for i in range(0, mesh.get_nb_faces()):
        print(i, mesh.get_face(i))
    
    faces = np.array(mesh.get_all_faces())
    print(faces)
    print(faces.shape, faces.dtype)

    if not os.path.exists("output"):
        os.mkdir("output")
    save_pymesh_ply('./output/cube.ply', mesh, "c'est un test d'écriture")