import py_skeletal_structures

from py_skeletal_structures import PyMesh3D, load_pymesh_obj, save_pymesh_ply


if __name__ == "__main__":
    mesh = load_pymesh_obj('./resources/cube.obj')

    print("nb vertices:", mesh.get_nb_vertices())
    for i in range(0, mesh.get_nb_vertices()):
        print(i, mesh.get_vertex(i))

    print("nb faces:", mesh.get_nb_faces())
    for i in range(0, mesh.get_nb_faces()):
        print(i, mesh.get_face(i))

    save_pymesh_ply('./resources/cube.ply', mesh, "c'est un test d'écriture")