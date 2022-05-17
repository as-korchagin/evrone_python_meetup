#include <Python.h>
#include <math.h>


const static double EARTH_RADIUS_M = 6372800;


static inline double _deg2rad(double deg) {
    return deg * M_PI / 180.0;
}


static inline double _haversine_m(
    double lon1deg,
    double lat1deg,
    double lon2deg,
    double lat2deg
) {
    double lon1rad = _deg2rad(lon1deg);
    double lat1rad = _deg2rad(lat1deg);
    double lon2rad = _deg2rad(lon2deg);
    double lat2rad = _deg2rad(lat2deg);

    double dlon = lon2rad - lon1rad;
    double dlat = lat2rad - lat1rad;

    double c = asin(
        sqrt(
            pow(sin(dlat / 2), 2) + cos(lat1rad) * cos(lat2rad) * pow(sin(dlon / 2), 2)
        )
    );
    return c * 2 * EARTH_RADIUS_M;
}


static PyObject * haversine_m(PyObject *self, PyObject *args)
{
    double lon1deg, lat1deg, lon2deg, lat2deg;
    double distance_m;

    if (!PyArg_ParseTuple(args, "dddd", &lon1deg, &lat1deg, &lon2deg, &lat2deg))
        return NULL;

    distance_m = _haversine_m(lon1deg, lat1deg, lon2deg, lat2deg);
    return PyFloat_FromDouble(distance_m);
}


static PyMethodDef haversine_methods[] = {
    {
        "haversine_m",
        haversine_m,
        METH_VARARGS,
        "(lon1, lat1, lon2, lat2)(all degrees) -> distance(m)\n computes haversine(great circle) distance\n"
    },
    {NULL, NULL, 0, NULL}        /* Sentinel */
};

static struct PyModuleDef haversine = {
    PyModuleDef_HEAD_INIT,
    "haversine",
    "still nothing to explain\n",
    -1,
    haversine_methods
};

PyMODINIT_FUNC PyInit_haversine(void)
{
    return PyModule_Create(&haversine);
}
