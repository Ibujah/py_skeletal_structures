
from py_skeletal_structures.simplicial2 import build_from_triangle_list


if __name__ == "__main__":
    tri_list = [[0,1,2], [1,0,3], [2,1,3],[0,2,3]]
    simpl = build_from_triangle_list(tri_list, True)

    nod0 = simpl.find_node(0)
    nod3 = simpl.find_node(3)

    print(nod0.to_string())
    print(nod3.to_string())
    print(len(nod0.halfedges()))

    print(simpl.get_nb_halfedges())
    print(simpl.get_nb_triangles())

    for i in range(0, simpl.get_nb_triangles()):
        tri = simpl.get_triangle_from_index(i)
        print(tri.to_string())
        for he in tri.halfedges():
            print(he.to_string(), he.opposite().to_string())
