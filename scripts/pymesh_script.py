import py_skeletal_structures

from py_skeletal_structures import PyMesh3D


if __name__ == "__main__":
    mesh = PyMesh3D()

    print("nb vertices:", mesh.get_nb_vertices())
    mesh.insert_vertex([1.0,1.0,0.])
    mesh.insert_vertex([0.0,1.0,0.])
    mesh.insert_vertex([1.0,0.0,0.])
    print("nb vertices:", mesh.get_nb_vertices())

    print("nb faces:", mesh.get_nb_faces())
    mesh.insert_face([0, 1, 2])
    print("nb faces:", mesh.get_nb_faces())

    try:
        print(mesh.get_vertex(0))
    except Exception as e:
        print(e)

    try:
        print(mesh.get_face(0))
    except Exception as e:
        print(e)
